#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let run_address = "[::1]:1284".parse().unwrap();
    let external_interface = impulse_interface_grpc::external::External::default();
    let internal_interface = impulse_interface_grpc::internal::Internal::default();

    println!("starting service on {}", run_address);

    impulse_interface_grpc::Server::builder()
        .add_service(impulse_interface_grpc::external::InterfaceServer::new(
            external_interface,
        ))
        .add_service(impulse_interface_grpc::internal::InterfaceServer::new(
            internal_interface,
        ))
        .serve(run_address)
        .await?;

    Ok(())
}
