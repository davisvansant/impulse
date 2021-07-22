fn main() -> Result<(), Box<dyn std::error::Error>> {
    runner()?;
    interface()?;
    Ok(())
}

fn runner() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../proto/impulse/impulse_runner.proto");
    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .out_dir("../proto/")
        .compile(
            &["../proto/impulse/impulse_runner.proto"],
            &["../proto/impulse"],
        )?;
    Ok(())
}

fn interface() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../proto/impulse/impulse_interface.proto");
    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .out_dir("../proto")
        .compile(
            &["../proto/impulse/impulse_interface.proto"],
            &["../proto/impulse"],
        )?;
    Ok(())
}
