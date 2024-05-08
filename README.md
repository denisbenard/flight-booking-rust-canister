## Flight Booking System Documentation

### Overview
The Flight Booking System is a decentralized application built on the Internet Computer (IC) platform. It allows users to manage flights, passengers, and bookings efficiently. The system offers functionalities for adding, deleting, updating, and querying flights, passengers, and bookings, as well as searching for flights and checking flight availability.

### Dependencies
- `serde`: Serialization and deserialization library for Rust.
- `candid`: Candid serialization and deserialization library.
- `ic_stable_structures`: Library providing stable data structures for the Internet Computer.
- `std`: Standard library for Rust.

### Data Structures
#### Structs
1. `Flight`: Represents a flight with fields including ID, airline, flight number, origin, destination, departure time, arrival time, capacity, and available seats.
2. `Passenger`: Represents a passenger with fields including ID, name, and email.
3. `Booking`: Represents a booking with fields including ID, flight ID, and passenger ID.

#### Enums
- `Error`: Represents possible error types including NotFound and InvalidInput.

### Functions
#### CRUD Operations for Flights
- `add_flight`: Add a new flight to the system.
- `delete_flight`: Delete a flight from the system.
- `update_flight`: Update details of an existing flight.
- `get_flight`: Get details of a specific flight.
- `search_flights`: Search flights based on criteria.
- `check_flight_availability`: Check availability of seats for a flight.

#### CRUD Operations for Passengers
- `add_passenger`: Add a new passenger to the system.
- `delete_passenger`: Delete a passenger from the system.
- `update_passenger`: Update details of an existing passenger.
- `get_passenger`: Get details of a specific passenger.

#### CRUD Operations for Bookings
- `book_flight`: Book a flight for a passenger.
- `cancel_booking`: Cancel a booking.

### Usage
The Flight Booking System provides a user-friendly interface for managing flight bookings. Users can seamlessly perform CRUD operations on flights, passengers, and bookings using the provided functions. Additionally, they can search for flights based on specific criteria and check the availability of seats for a flight.
### Requirements
* rustc 1.64 or higher
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```
* rust wasm32-unknown-unknown target
```bash
$ rustup target add wasm32-unknown-unknown
```
* candid-extractor
```bash
$ cargo install candid-extractor
```
* install `dfx`
```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
$ dfx start --background
```

If you want to start working on your project right away, you might want to try the following commands:

```bash
$ cd flight-booking-rust-canister/
$ dfx help
$ dfx canister --help
```

## Update dependencies

update the `dependencies` block in `/src/{canister_name}/Cargo.toml`:
```
[dependencies]
candid = "0.9.9"
ic-cdk = "0.11.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
ic-stable-structures = { git = "https://github.com/lwshang/stable-structures.git", branch = "lwshang/update_cdk"}
```

## did autogenerate

Add this script to the root directory of the project:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh
```

Update line 16 with the name of your canister:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh#L16
```

After this run this script to generate Candid.
Important note!

You should run this script each time you modify/add/remove exported functions of the canister.
Otherwise, you'll have to modify the candid file manually.

Also, you can add package json with this content:
```
{
    "scripts": {
        "generate": "./did.sh && dfx generate",
        "gen-deploy": "./did.sh && dfx generate && dfx deploy -y"
      }
}
```

and use commands `npm run generate` to generate candid or `npm run gen-deploy` to generate candid and to deploy a canister.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
$ dfx start --background

# Deploys your canisters to the replica and generates your candid interface
$ dfx deploy
```
