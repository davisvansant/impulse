#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ipv4_addr = std::net::Ipv4Addr::new(0, 0, 0, 0);
    let port = 1284;
    let socket_addr = std::net::SocketAddr::new(std::net::IpAddr::V4(ipv4_addr), port);

    let (tx, _) = tokio::sync::broadcast::channel(1);
    let sender_clone = tx.clone();

    let (response_tx, _) = tokio::sync::broadcast::channel(1);
    let response_sender_clone = response_tx.clone();

    let external_interface =
        impulse_interface_external::External::init(tx, response_sender_clone).await?;
    let internal_interface =
        impulse_interface_internal::Internal::init(sender_clone, response_tx).await?;

    println!(
        ":: i m p u l s e _ i n t e r f a c e > Launching system | {}",
        &socket_addr,
    );

    println!(
        ":: i m p u l s e _ i n t e r f a c e > System Version | {}",
        &external_interface.version,
    );

    println!(
        ":: i m p u l s e _ i n t e r f a c e > System id | {}",
        &internal_interface.system_id,
    );

    let ctrl_c = async move {
        println!(":: i m p u l s e _ i n t e r f a c e > Running...");
        tokio::signal::ctrl_c().await.unwrap();
        println!(":: i m p u l s e _ i n t e r f a c e > Shutting down...");
    };

    tonic::transport::Server::builder()
        .add_service(impulse_interface_external::InterfaceServer::new(
            external_interface,
        ))
        .add_service(impulse_interface_internal::InterfaceServer::new(
            internal_interface,
        ))
        .serve_with_shutdown(socket_addr, ctrl_c)
        .await?;

    Ok(())
}