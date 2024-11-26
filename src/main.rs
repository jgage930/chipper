pub mod core;

use anyhow::Result;
use dotenv::dotenv;
use log::{debug, info};

fn main() -> Result<()> {
    env_logger::init();
    dotenv().expect("Failed to load enviroment variables.  Did you create a .env file?");

    debug!("This is a test debug message...");
    info!("This is a test info message...");

    println!("Hello, world!");

    Ok(())
}