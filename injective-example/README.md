# Injective Protocol gRPC Example

This is a simple example of a gRPC server and client for the Injective Protocol exchange service, focusing on spot market orders.

## Setup

1. Make sure you have Rust and Cargo installed.
2. Ensure the generated Rust code from Injective Protocol is in the `rust/proto/` directory.
3. Run `cargo build` to compile the project.

## Running the Server

To start the gRPC server:

```bash
cargo run --bin server
```

The server will listen on `[::1]:50051` (localhost:50051).

## Running the Client

To run the client and send a spot market order:

```bash
cargo run --bin client
```

The client will connect to the server and send a sample spot market order.

## Project Structure

- `src/server.rs`: Implements the gRPC server for handling spot market orders.
- `src/client.rs`: Implements a gRPC client that sends spot market orders.
- `rust/proto/`: Contains the generated Rust code from Injective Protocol.

## Notes

This is a simplified example. In a real-world application:

1. You would need to include proper authentication and validation.
2. You would interact with the actual Injective blockchain network.
3. You would handle errors more robustly.
4. You would implement all the methods required by the service.

## Dependencies

- `tonic`: gRPC implementation for Rust
- `prost`: Protocol Buffers implementation for Rust
- `tokio`: Asynchronous runtime for Rust 