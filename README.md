# ICUtil - Internet Computer Flow Sensor Data Management

A robust Internet Computer canister for collecting, storing, and analyzing flow sensor data with comprehensive error handling and data validation.

## Features

- **Real-time Flow Data Collection**: Record flow sensor readings with timestamps and optional device identification
- **Data Validation**: Input validation for flow rates with configurable limits (0-1000 L/min)
- **Statistical Analysis**: Calculate averages, min/max values, standard deviation, and total volume
- **Efficient Storage**: Uses VecDeque for optimal performance with automatic data rotation (max 1000 readings)
- **Comprehensive Error Handling**: Proper error types and Result-based API responses
- **Data Export**: JSON export functionality for external analysis
- **Admin Functions**: Clear all data functionality for maintenance

## Data Structure

### FlowReading
```rust
struct FlowReading {
    timestamp: u64,           // UNIX epoch time in seconds
    flow_rate: f64,          // Flow rate in liters per minute
    device_id: Option<String> // Optional device identifier
}
```

### FlowStatistics
```rust
struct FlowStatistics {
    count: usize,        // Number of readings
    average: f64,        // Average flow rate
    min: f64,           // Minimum flow rate
    max: f64,           // Maximum flow rate
    std_deviation: f64, // Standard deviation
    total_volume: f64,  // Total volume measured
}
```

## API Reference

### Update Functions

#### `record_flow_data(flow_rate: f64, device_id: Option<String>) -> Result<String, FlowError>`
Records a new flow sensor reading with validation.

**Parameters:**
- `flow_rate`: Flow rate in L/min (must be between 0-1000)
- `device_id`: Optional device identifier

**Returns:** Success message or validation error

#### `clear_all_readings() -> Result<String, FlowError>`
Clears all stored readings (admin function).

### Query Functions

#### `get_recent_readings(count: usize) -> Result<Vec<FlowReading>, FlowError>`
Retrieves the most recent flow readings.

**Parameters:**
- `count`: Number of readings to retrieve (max 1000)

#### `get_average_flow_rate() -> Result<f64, FlowError>`
Calculates the average flow rate across all readings.

#### `get_flow_statistics() -> Result<FlowStatistics, FlowError>`
Returns comprehensive statistics including average, min, max, standard deviation, and total volume.

#### `export_all_readings() -> Result<String, FlowError>`
Exports all readings as JSON string for external analysis.

#### `get_readings_count() -> Result<usize, FlowError>`
Returns the total number of stored readings.

## Error Handling

The system uses a comprehensive error handling approach with the `FlowError` enum:

- `InvalidFlowRate(String)`: Flow rate validation errors
- `StorageError(String)`: Storage operation failures
- `DataNotFound`: No data available for the requested operation

## Configuration

- **MAX_READINGS**: 1000 (automatic rotation)
- **MIN_FLOW_RATE**: 0.0 L/min
- **MAX_FLOW_RATE**: 1000.0 L/min

## Quick Start

To get started with the Internet Computer development environment, see the following documentation:

- [Quick Start](https://internetcomputer.org/docs/current/developer-docs/setup/deploy-locally)
- [SDK Developer Tools](https://internetcomputer.org/docs/current/developer-docs/setup/install)
- [Rust Canister Development Guide](https://internetcomputer.org/docs/current/developer-docs/backend/rust/)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://internetcomputer.org/docs/current/developer-docs/backend/candid/)

## Usage Examples

### Recording Flow Data
```bash
# Record a flow reading of 25.5 L/min
dfx canister call icutil_backend record_flow_data '(25.5, null)'

# Record with device ID
dfx canister call icutil_backend record_flow_data '(30.2, opt "sensor-001")'
```

### Querying Data
```bash
# Get the last 10 readings
dfx canister call icutil_backend get_recent_readings '(10)'

# Get average flow rate
dfx canister call icutil_backend get_average_flow_rate '()'

# Get comprehensive statistics
dfx canister call icutil_backend get_flow_statistics '()'

# Get total number of readings
dfx canister call icutil_backend get_readings_count '()'

# Export all data as JSON
dfx canister call icutil_backend export_all_readings '()'
```

### Administrative Functions
```bash
# Clear all readings (use with caution)
dfx canister call icutil_backend clear_all_readings '()'
```

## Project Structure

```
icutil/
├── src/
│   ├── icutil_backend/          # Rust canister backend
│   │   ├── src/
│   │   │   └── lib.rs          # Main canister logic
│   │   ├── Cargo.toml          # Rust dependencies
│   │   └── icutil_backend.did  # Candid interface
│   └── icutil_frontend/         # Frontend application
├── dfx.json                     # DFX configuration
├── Cargo.toml                   # Workspace configuration
└── README.md                    # This file
```

## Getting Started

If you want to start working on your project right away, you might want to try the following commands:

```bash
cd icutil/
dfx help
dfx canister --help
```

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.

If you have made changes to your backend canister, you can generate a new candid interface with

```bash
npm run generate
```

at any time. This is recommended before starting the frontend development server, and will be run automatically any time you run `dfx deploy`.

If you are making frontend changes, you can start a development server with

```bash
npm start
```

Which will start a server at `http://localhost:8080`, proxying API requests to the replica at port 4943.

### Note on frontend environment variables

If you are hosting frontend code somewhere without using DFX, you may need to make one of the following adjustments to ensure your project does not fetch the root key in production:

- set`DFX_NETWORK` to `ic` if you are using Webpack
- use your own preferred method to replace `process.env.DFX_NETWORK` in the autogenerated declarations
  - Setting `canisters -> {asset_canister_id} -> declarations -> env_override to a string` in `dfx.json` will replace `process.env.DFX_NETWORK` with the string in the autogenerated declarations
- Write your own `createActor` constructor
