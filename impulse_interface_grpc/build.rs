fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../proto/impulse/impulse.proto");
    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .compile(&["../proto/impulse/impulse.proto"], &["../proto/impulse"])?;
    Ok(())
}
