use system::actuator_client::Internal;
use system::actuator_engine::Engine;
use system::IMPULSE_ACTUATOR;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let ctrl_c = tokio::spawn(async move {
    //     println!(":: i m p u l s e _ a c t u a t o r > Running...");
    //     tokio::signal::ctrl_c().await.unwrap();
    //     println!(":: i m p u l s e _ a c t u a t o r > Shutting down...");
    // });

    let endpoint = "http://[::1]:1284";
    println!("{} connecting | {}", IMPULSE_ACTUATOR, &endpoint);

    let mut internal_client = Internal::init(endpoint).await?;
    println!(
        "{} node id | {}",
        IMPULSE_ACTUATOR, &internal_client.node_id,
    );

    let mut engine = Engine::init().await?;
    println!("{} engine active | {}", IMPULSE_ACTUATOR, &engine.active);

    let register = internal_client.register().await?;
    println!(
        "{} endpoint system id | {}",
        IMPULSE_ACTUATOR,
        &register.get_ref().system_id,
    );

    let mut controller = internal_client.controller().await?;
    println!("{} awaiting tasks . . .", IMPULSE_ACTUATOR);

    while let Some(task) = controller.get_mut().message().await? {
        match task.action {
            1 => {
                println!("start a vm {:?}", task);
                engine.launch_vm(&task.id).await?;
                internal_client.launch_result(&task.id).await?;
            }
            2 => {
                println!("shutdown a vm {:?}", task);
                engine.shutdown_vm(&task.id).await?;
                internal_client.shutdown_result(&task.id).await?;
            }
            _ => (),
        }
    }

    Ok(())
}
