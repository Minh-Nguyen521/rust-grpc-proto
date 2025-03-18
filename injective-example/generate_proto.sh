#!/bin/bash
set -e

# Generate Rust code from proto file using protoc
# This is a fallback if tonic-build doesn't work for some reason
# protoc --rust_out=src/generated --grpc_out=src/generated --plugin=protoc-gen-grpc=`which grpc_rust_plugin` proto/hello.proto
# need to create src/generated directory first

# Note: The above method requires manual installation of protoc and grpc_rust_plugin
# In practice, using cargo build is sufficient as tonic-build handles everything
echo "Building project to generate proto files..."
cd injective-example
unzip rust_protos

echo "Proto files generated successfully!" 