// Generate Rust gRPC code from proto file.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/openconfig.proto")?;
    Ok(())
}
