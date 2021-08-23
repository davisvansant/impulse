use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use tokio::sync::broadcast::channel;

use system::external_interface::{External, InterfaceServer as ExternalInterfaceServer};
use system::internal_interface::{InterfaceServer as InternalInterfaceServer, Internal};
use system::IMPULSE_INTERFACE;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ipv4_addr = Ipv4Addr::new(0, 0, 0, 0);
    let port = 1284;
    let socket_addr = SocketAddr::new(IpAddr::V4(ipv4_addr), port);

    let (task_sender, _) = channel(4);
    let task_sender_clone = task_sender.clone();

    let (launch_result_sender, _) = channel(4);
    let launch_result_sender_clone = launch_result_sender.clone();

    let (shutdown_result_sender, _) = channel(4);
    let shutdown_result_sender_clone = shutdown_result_sender.clone();

    let external_interface = External::init(
        task_sender,
        launch_result_sender_clone,
        shutdown_result_sender_clone,
    )
    .await?;

    let internal_interface = Internal::init(
        task_sender_clone,
        launch_result_sender,
        shutdown_result_sender,
    )
    .await?;

    println!("{} Launching system | {}", IMPULSE_INTERFACE, &socket_addr);

    println!(
        "{} System Version | {}",
        IMPULSE_INTERFACE, &external_interface.version,
    );

    println!(
        "{} System id | {}",
        IMPULSE_INTERFACE, &internal_interface.system_id,
    );

    let ctrl_c = async move {
        println!("{} Running...", IMPULSE_INTERFACE);
        tokio::signal::ctrl_c().await.unwrap();
        println!("{} > Shutting down...", IMPULSE_INTERFACE);
    };

    tonic::transport::Server::builder()
        .add_service(ExternalInterfaceServer::new(external_interface))
        .add_service(InternalInterfaceServer::new(internal_interface))
        .serve_with_shutdown(socket_addr, ctrl_c)
        .await?;

    Ok(())
}
