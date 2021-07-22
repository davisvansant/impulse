fn main() -> Result<(), Box<dyn std::error::Error>> {
    build_proto("external")?;
    build_proto("internal")?;
    Ok(())
}

fn build_proto(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let proto_file = format!("../proto/impulse/impulse_{}_v010.proto", name);
    println!("cargo:rerun-if-changed={}", proto_file);
    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .out_dir("../proto")
        .compile(&[proto_file.as_str()], &["../proto/impulse"])?;
    Ok(())
}
