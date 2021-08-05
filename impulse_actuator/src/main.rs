#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let ctrl_c = tokio::spawn(async move {
    //     println!(":: i m p u l s e _ a c t u a t o r > Running...");
    //     tokio::signal::ctrl_c().await.unwrap();
    //     println!(":: i m p u l s e _ a c t u a t o r > Shutting down...");
    // });

    let endpoint = "http://[::1]:1284";
    println!(
        ":: i m p u l s e _ a c t u a t o r > connecting | {}",
        &endpoint,
    );

    let mut internal_client = impulse_actuator_internal::Internal::init(endpoint).await?;
    println!(
        ":: i m p u l s e _ a c t u a t o r >  node id | {}",
        &internal_client.node_id,
    );

    let mut engine = impulse_actuator_engine::Engine::init().await?;
    println!(
        ":: i m p u l s e _ a c t u a t o r > engine active | {}",
        &engine.active,
    );

    let register = internal_client.register().await?;
    println!(
        ":: i m p u l s e _ a c t u a t o r > endpoint system id | {}",
        &register.get_ref().system_id,
    );

    let mut controller = internal_client.controller().await?;
    println!(":: i m p u l s e _ a c t u a t o r > awaiting tasks |");

    while let Some(task) = controller.get_mut().message().await? {
        match task.action {
            1 => {
                println!("start a vm {:?}", task);
                engine.launch_vm(&task.id).await?;
            }
            2 => {
                println!("shutdown a vm {:?}", task);
                engine.shutdown_vm().await?;
            }
            _ => (),
        }
    }

    Ok(())
}
