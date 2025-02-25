mod config;
mod listener;

use crate::config::Config;
use crate::listener::Listener;
use tokio;

#[tokio::main]
async fn main() -> web3::Result<()> {
    // Load configuration from command line args
    let config = Config::from_args();

    // Initialize the listener with the given config
    let listener = Listener::new(&config).await?;

    // Start listening for transactions
    println!("Listening for transactions to {:?}", config.target_address);
    listener.listen().await?;

    Ok(())
}
