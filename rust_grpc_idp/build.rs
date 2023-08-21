fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let proto_file = "/Users/ek_solution/BASE/CODES/projects/sso_micro_arc/proto/identity.proto";
    tonic_build::configure()
        .build_server(true)
        .compile(&["proto/identity.proto"], &["proto"])?;

    Ok(())
}
