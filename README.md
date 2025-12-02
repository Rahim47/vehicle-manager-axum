# Vehicle Manager API

A RESTful API built with Rust and Axum for managing vehicle information. This project demonstrates building a web API using modern Rust async/await patterns with the Axum web framework.

## Features

- **GET** endpoint to retrieve vehicle information
- **POST** endpoint to create new vehicles with automatic UUID generation
- Support for query parameters and JSON body payloads
- Customer information handling (in development)
- Built with async/await for high performance

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- Cargo (comes with Rust)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/YOUR_USERNAME/vehicle-manager-axum.git
cd vehicle-manager-axum
```

2. Build the project:
```bash
cargo build --release
```

3. Run the server:
```bash
cargo run
```

The server will start on `http://0.0.0.0:4734`

## API Endpoints

### GET /vehicle

Retrieves a sample vehicle with auto-generated UUID.

**Response:**
```json
{
  "manufacturer": "Toyota",
  "model": "Camry",
  "year": 2022,
  "id": "550e8400-e29b-41d4-a716-446655440000"
}
```

### POST /vehicle

Creates a new vehicle. Accepts both query parameters and JSON body.

**Query Parameters:**
- `manufacturer` (string): Vehicle manufacturer
- `model` (string): Vehicle model
- `year` (u32): Manufacturing year
- `first_name` (string, optional): Customer first name
- `last_name` (string, optional): Customer last name

**JSON Body:**
```json
{
  "manufacturer": "Honda",
  "model": "Civic",
  "year": 2026
}
```

**Response:**
Returns the vehicle with an auto-generated UUID:
```json
{
  "manufacturer": "Honda",
  "model": "Civic",
  "year": 2026,
  "id": "550e8400-e29b-41d4-a716-446655440000"
}
```

## Testing

### Using PowerShell

The project includes PowerShell test scripts in `api-tests.ps1`:

```powershell
# Run all tests
.\api-tests.ps1
```

### Using curl

**GET request:**
```bash
curl http://localhost:4734/vehicle
```

**POST request with JSON body:**
```bash
curl -X POST http://localhost:4734/vehicle \
  -H "Content-Type: application/json" \
  -d '{"manufacturer":"Honda","model":"Civic","year":2026}'
```

**POST request with query parameters:**
```bash
curl -X POST "http://localhost:4734/vehicle?manufacturer=Toyota&model=Camry&year=2025"
```

## Project Structure

```
vehicle-manager-axum/
├── src/
│   ├── main.rs          # Application entry point and router setup
│   └── vehicle.rs       # Vehicle and Customer structs, handlers
├── Cargo.toml           # Project dependencies and metadata
├── api-tests.ps1        # PowerShell API test scripts
└── README.md            # This file
```

## Technologies Used

- **Rust** - Systems programming language
- **Axum** - Web application framework for Rust
- **Tokio** - Async runtime for Rust
- **Serde** - Serialization/deserialization framework
- **UUID** - Unique identifier generation

## Development

### Running in Development Mode

```bash
cargo run
```

### Building for Production

```bash
cargo build --release
```

The optimized binary will be in `target/release/vehicle-manager-axum`

## License

This project is open source and available under the MIT License.

## Contributing

Contributions, issues, and feature requests are welcome! Feel free to check the issues page.

## Author

Rahim Sekkouti - [Rahim47](https://github.com/Rahim47)

