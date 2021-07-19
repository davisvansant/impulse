use impulse_actuator::Actuator;
use impulse_interface::Interface;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Actuator::init().await?;
    Interface::init().await?;
    Ok(())
}
