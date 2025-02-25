mod config;
mod listener;

use crate::config::Config;
use crate::listener::Listener;
use tokio;

#[tokio::main]
async fn main() {
    // Load configuration from command line args
    let config = match Config::from_args() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error parsing arguments: {}", e);
            return;
        }
    };

    // Initialize the listener with the given config
    let listener = Listener::new(&config);

    // Start listening for transactions
    if let Err(e) = listener.listen().await {
        eprintln!("Error listening for transactions: {}", e);
        return;
    }
}
