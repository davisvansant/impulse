// use impulse_actuator::Actuator;
// use impulse_interface::Interface;

use clap::{App, SubCommand};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let name = env!("CARGO_PKG_NAME");
    let author = env!("CARGO_PKG_AUTHORS");
    let version = env!("CARGO_PKG_VERSION");

    let start = SubCommand::with_name("start")
        .display_order(1)
        .about("Start");
    let shutdown = SubCommand::with_name("shutdown")
        .display_order(2)
        .about("Shutdown");

    let impulse = App::new(name)
        .author(author)
        .version(version)
        .about("MicroVM runner")
        .subcommand(start)
        .subcommand(shutdown)
        .get_matches();

    match impulse.subcommand_name() {
        Some("start") => {
            println!("do starty things");

            // Actuator::init().await?;
            // Interface::init().await?;
        }
        Some("shutdown") => println!("do shutdowny things"),
        _ => println!("{}", impulse.usage()),
    }

    Ok(())
}
