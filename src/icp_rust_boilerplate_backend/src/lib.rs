#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

// Define type aliases for memory management
type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// Define the structure for a flight
#[derive(candid::CandidType, Serialize, Deserialize, Clone)]
struct Flight {
    id: u64,
    airline: String,
    flight_number: String,
    origin: String,
    destination: String,
    departure_time: u64, // Unix timestamp
    arrival_time: u64,   // Unix timestamp
    capacity: u32,
    available_seats: u32,
}

// Define the structure for a passenger
#[derive(candid::CandidType, Serialize, Deserialize, Clone)]
struct Passenger {
    id: u64,
    name: String,
    email: String,
    // Add any other relevant fields for the passenger
}

// Define the structure for a booking
#[derive(candid::CandidType, Serialize, Deserialize, Clone)]
struct Booking {
    id: u64,
    flight_id: u64,
    passenger_id: u64,
}

// Implement serialization and deserialization for Flight
impl Storable for Flight {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implement bounds for Flight serialization
impl BoundedStorable for Flight {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Implement serialization and deserialization for Passenger
impl Storable for Passenger {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implement bounds for Passenger serialization
impl BoundedStorable for Passenger {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Implement serialization and deserialization for Booking
impl Storable for Booking {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implement bounds for Booking serialization
impl BoundedStorable for Booking {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Thread-local storage for memory management, ID counter, flight storage, passenger storage, and booking storage
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static FLIGHT_STORAGE: RefCell<StableBTreeMap<u64, Flight, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static PASSENGER_STORAGE: RefCell<StableBTreeMap<u64, Passenger, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static BOOKING_STORAGE: RefCell<StableBTreeMap<u64, Booking, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));
}

// Define the possible errors
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    InvalidInput { msg: String },
}

// Implement CRUD operations for flights
#[ic_cdk::update]
fn add_flight(airline: String, flight_number: String, origin: String, destination: String, departure_time: u64, arrival_time: u64, capacity: u32) -> Result<Flight, Error> {
    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1);
        current_value + 1
    });

    let flight = Flight {
        id,
        airline,
        flight_number,
        origin,
        destination,
        departure_time,
        arrival_time,
        capacity,
        available_seats: capacity, // Initially all seats are available
    };

    FLIGHT_STORAGE.with(|storage| storage.borrow_mut().insert(id, flight.clone()));
    Ok(flight)
}

#[ic_cdk::update]
fn delete_flight(id: u64) -> Result<(), Error> {
    match FLIGHT_STORAGE.with(|storage| storage.borrow_mut().remove(&id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Flight with id={} not found", id),
        }),
    }
}

// Implement CRUD operations for passengers
#[ic_cdk::update]
fn add_passenger(name: String, email: String) -> Result<Passenger, Error> {
    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1);
        current_value + 1
    });

    let passenger = Passenger {
        id,
        name,
        email,
    };

    PASSENGER_STORAGE.with(|storage| storage.borrow_mut().insert(id, passenger.clone()));
    Ok(passenger)
}

#[ic_cdk::update]
fn delete_passenger(id: u64) -> Result<(), Error> {
    match PASSENGER_STORAGE.with(|storage| storage.borrow_mut().remove(&id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Passenger with id={} not found", id),
        }),
    }
}

// Implement CRUD operations for bookings
#[ic_cdk::update]
fn book_flight(flight_id: u64, passenger_id: u64) -> Result<Booking, Error> {
    // Check if the flight exists
    let flight = match get_flight(flight_id) {
        Ok(f) => f,
        Err(_) => return Err(Error::NotFound {
            msg: format!("Flight with id={} not found", flight_id),
        }),
    };

    // Check if the passenger exists
    let _ = match get_passenger(passenger_id) {
        Ok(p) => p,
        Err(_) => return Err(Error::NotFound {
            msg: format!("Passenger with id={} not found", passenger_id),
        }),
    };

    // Check if there are available seats
    if flight.available_seats == 0 {
        return Err(Error::InvalidInput {
            msg: "No available seats for this flight".to_string(),
        });
    }

    // Decrement available seats
    let mut updated_flight = flight.clone();
    updated_flight.available_seats -= 1;
    update_flight(
        flight_id,
        updated_flight.airline.clone(),
        updated_flight.flight_number.clone(),
        updated_flight.origin.clone(),
        updated_flight.destination.clone(),
        updated_flight.departure_time,
        updated_flight.arrival_time,
        updated_flight.capacity,
    )?;

    // Create booking
    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1);
        current_value + 1
    });

    let booking = Booking {
        id,
        flight_id,
        passenger_id,
    };

    // Save booking
    BOOKING_STORAGE.with(|storage| storage.borrow_mut().insert(id, booking.clone()));

    Ok(booking)
}

#[ic_cdk::update]
fn cancel_booking(booking_id: u64) -> Result<(), Error> {
    // Check if the booking exists
    match get_booking(booking_id) {
        Ok(booking) => {
            // Increment available seats for the flight
            let mut flight = get_flight(booking.flight_id)?;
            flight.available_seats += 1;
            update_flight(
                booking.flight_id,
                flight.airline.clone(),
                flight.flight_number.clone(),
                flight.origin.clone(),
                flight.destination.clone(),
                flight.departure_time,
                flight.arrival_time,
                flight.capacity,
            )?;

            // Remove booking
            BOOKING_STORAGE.with(|storage| storage.borrow_mut().remove(&booking_id));
            Ok(())
        },
        Err(e) => Err(e),
    }
}

// Implement query operations for the flight booking system
#[ic_cdk::query]
fn get_flight(id: u64) -> Result<Flight, Error> {
    match FLIGHT_STORAGE.with(|storage| storage.borrow().get(&id)) {
        Some(flight) => Ok(flight.clone()),
        None => Err(Error::NotFound {
            msg: format!("Flight with id={} not found", id),
        }),
    }
}

#[ic_cdk::query]
fn get_passenger(id: u64) -> Result<Passenger, Error> {
    match PASSENGER_STORAGE.with(|storage| storage.borrow().get(&id)) {
        Some(passenger) => Ok(passenger.clone()),
        None => Err(Error::NotFound {
            msg: format!("Passenger with id={} not found", id),
        }),
    }
}

#[ic_cdk::query]
fn get_booking(id: u64) -> Result<Booking, Error> {
    match BOOKING_STORAGE.with(|storage| storage.borrow().get(&id)) {
        Some(booking) => Ok(booking.clone()),
        None => Err(Error::NotFound {
            msg: format!("Booking with id={} not found", id),
        }),
    }
}

// Implement update operation for flights
#[ic_cdk::update]
fn update_flight(id: u64, airline: String, flight_number: String, origin: String, destination: String, departure_time: u64, arrival_time: u64, capacity: u32) -> Result<Flight, Error> {
    match FLIGHT_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(flight) = storage.get(&id) {
            // Create a cloned copy of the flight to update
            let mut updated_flight = flight.clone();
            // Update the flight fields
            updated_flight.airline = airline;
            updated_flight.flight_number = flight_number;
            updated_flight.origin = origin;
            updated_flight.destination = destination;
            updated_flight.departure_time = departure_time;
            updated_flight.arrival_time = arrival_time;
            updated_flight.capacity = capacity;
            // Replace the old flight with the updated one
            storage.insert(id, updated_flight.clone());
            Ok(updated_flight)
        } else {
            Err(Error::NotFound {
                msg: format!("Flight with id={} not found", id),
            })
        }
    }) {
        Ok(flight) => Ok(flight),
        Err(e) => Err(e),
    }
}

// Implement update operation for passengers
#[ic_cdk::update]
fn update_passenger(id: u64, name: String, email: String) -> Result<Passenger, Error> {
    match PASSENGER_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(passenger) = storage.get(&id) {
            // Create a cloned copy of the passenger to update
            let mut updated_passenger = passenger.clone();
            // Update the passenger fields
            updated_passenger.name = name;
            updated_passenger.email = email;
            // Replace the old passenger with the updated one
            storage.insert(id, updated_passenger.clone());
            Ok(updated_passenger)
        } else {
            Err(Error::NotFound {
                msg: format!("Passenger with id={} not found", id),
            })
        }
    }) {
        Ok(passenger) => Ok(passenger),
        Err(e) => Err(e),
    }
}

// Search Flights by Criteria
#[ic_cdk::query]
fn search_flights(criteria: String) -> Vec<Flight> {
    let mut result = Vec::new();
    FLIGHT_STORAGE.with(|storage| {
        let storage = storage.borrow();
        for (_, flight) in storage.iter() {
            // Check if the flight matches the search criteria
            if flight.airline.contains(&criteria)
                || flight.flight_number.contains(&criteria)
                || flight.origin.contains(&criteria)
                || flight.destination.contains(&criteria)
            {
                result.push(flight.clone());
            }
        }
    });
    result
}

// Flight Availability Check
#[ic_cdk::query]
fn check_flight_availability(id: u64) -> Result<bool, Error> {
    match get_flight(id) {
        Ok(flight) => Ok(flight.available_seats > 0),
        Err(e) => Err(e),
    }
}

// List all flights
#[ic_cdk::query]
fn list_all_flights() -> Vec<Flight> {
    let mut flights = Vec::new();
    FLIGHT_STORAGE.with(|storage| {
        let storage = storage.borrow();
        for (_, flight) in storage.iter() {
            flights.push(flight.clone());
        }
    });
    flights
}

// List all passengers
#[ic_cdk::query]
fn list_all_passengers() -> Vec<Passenger> {
    let mut passengers = Vec::new();
    PASSENGER_STORAGE.with(|storage| {
        let storage = storage.borrow();
        for (_, passenger) in storage.iter() {
            passengers.push(passenger.clone());
        }
    });
    passengers
}

// List all bookings
#[ic_cdk::query]
fn list_all_bookings() -> Vec<Booking> {
    let mut bookings = Vec::new();
    BOOKING_STORAGE.with(|storage| {
        let storage = storage.borrow();
        for (_, booking) in storage.iter() {
            bookings.push(booking.clone());
        }
    });
    bookings
}

// Export the Candid interface
ic_cdk::export_candid!();
