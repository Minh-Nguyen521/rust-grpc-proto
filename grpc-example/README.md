# Rust gRPC Example

A simple example of using gRPC with Rust and Tonic.

## Project Structure

- `proto/hello.proto` - The protocol buffer definition
- `src/server.rs` - The gRPC server implementation
- `src/client.rs` - The gRPC client implementation
- `build.rs` - Rust build script to compile proto files
- `generate_proto.sh` - Shell script to generate proto files

## Usage

### Generate proto files

```bash
# Run the script
./generate_proto.sh
```

### Run the server

```bash
cargo run --bin server
```

### Run the client (in a different terminal)

```bash
cargo run --bin client
```

## Requirements

- Rust
- Cargo
- Protobuf compiler (protoc) - installed by tonic-build 