fn main() -> Result<(), Box<dyn std::error::Error>> {
    build_proto("external", "v010", false, true)?;
    build_proto("internal", "v010", false, true)?;
    Ok(())
}

fn build_proto(
    name: &str,
    version: &str,
    client: bool,
    server: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let proto_file = format!("../proto/impulse/impulse_{}_{}.proto", name, version);
    println!("cargo:rerun-if-changed={}", proto_file);
    tonic_build::configure()
        .build_client(client)
        .build_server(server)
        .out_dir("../proto")
        .compile(&[proto_file.as_str()], &["../proto/impulse"])?;
    Ok(())
}
