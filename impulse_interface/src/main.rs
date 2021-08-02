#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let run_address = "[::1]:1284".parse().unwrap();
    let (tx, _) = tokio::sync::broadcast::channel(1);
    let sender_clone = tx.clone();
    let external_interface = impulse_interface_grpc::external::External::init(tx).await?;
    let internal_interface = impulse_interface_grpc::internal::Internal::init(sender_clone).await?;

    println!(
        ":: i m p u l s e _ i n t e r f a c e > Launching system on {}",
        run_address,
    );

    let ctrl_c = async move {
        println!(":: i m p u l s e _ i n t e r f a c e > Running...");
        tokio::signal::ctrl_c().await.unwrap();
        println!(":: i m p u l s e _ i n t e r f a c e > Shutting down...");
    };

    impulse_interface_grpc::Server::builder()
        .add_service(impulse_interface_grpc::external::InterfaceServer::new(
            external_interface,
        ))
        .add_service(impulse_interface_grpc::internal::InterfaceServer::new(
            internal_interface,
        ))
        .serve_with_shutdown(run_address, ctrl_c)
        .await?;

    Ok(())
}
