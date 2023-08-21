fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let proto_file = "./proto/identity.proto";
    tonic_build::configure()
        .build_server(true)
        .compile(&["proto/identity.proto"], &["proto"])?;

    Ok(())
}
