fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/cosmos.msg.textual.v1.proto")?;
    Ok(())
} 