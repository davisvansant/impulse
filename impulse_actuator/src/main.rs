#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let ctrl_c = tokio::spawn(async move {
    //     println!(":: i m p u l s e _ a c t u a t o r > Running...");
    //     tokio::signal::ctrl_c().await.unwrap();
    //     println!(":: i m p u l s e _ a c t u a t o r > Shutting down...");
    // });

    // let actuator = Actuator::init().await?;
    let endpoint = "http://[::1]:1284";
    let mut internal_client = impulse_actuator_internal::Internal::init(endpoint).await?;

    internal_client.register().await?;

    let mut controller = internal_client.controller().await?;

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
