pub use internal_v010::interface_client::InterfaceClient;
pub use internal_v010::{NodeId, SystemId, Task};

// use impulse_actuator::Actuator;

mod internal_v010 {
    include!("../../proto/impulse.internal.v010.rs");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let ctrl_c = tokio::spawn(async move {
    //     println!(":: i m p u l s e _ a c t u a t o r > Running...");
    //     tokio::signal::ctrl_c().await.unwrap();
    //     println!(":: i m p u l s e _ a c t u a t o r > Shutting down...");
    // });

    let mut client = InterfaceClient::connect("http://[::1]:1284").await?;
    // let actuator = Actuator::init().await?;
    let node_id = String::from("some_node_id");

    let register_request = tonic::Request::new(NodeId {
        node_id: node_id.clone(),
    });

    let register_response = client.register(register_request).await?;

    println!("Registered! - {:?}", register_response);

    let controller_request = tonic::Request::new(NodeId {
        node_id: node_id.clone(),
    });

    let mut controller = client.controller(controller_request).await?;
    while let Some(task) = controller.get_mut().message().await? {
        match task.action {
            1 => {
                println!("start a vm {:?}", task);
                // actuator.start_vm().await?;
            }
            2 => {
                println!("shutdown a vm {:?}", task);
                // actuator.shutdown_vm().await?;
            }
            _ => (),
        }
    }

    Ok(())
}
