use clap::{Command, Arg};
use web3::types::Address;
use std::str::FromStr;

pub struct Config {
    pub ws_url: String,
    pub target_address: Address,
}

impl Config {
    pub fn from_args() -> Result<Self, String> {
        let matches = Command::new("Web3 Transaction Listener")
            .version("1.0")
            .author("Author <author@example.com>")
            .about("Listens for transactions to a specific address")
            .arg(
                Arg::new("url")
                    .long("url")
                    .short('u')
                    .default_value("ws://127.0.0.1:8545/")
                    .help("WebSocket URL of the Ethereum node"),
            )
            .arg(
                Arg::new("address")
                    .long("address")
                    .short('a')
                    .required(true)
                    .help("Ethereum address to listen for transactions to"),
            )
            .get_matches();

        let ws_url = matches
            .get_one::<String>("url")
            .map(|s| s.to_string())
            .unwrap_or_else(|| "ws://127.0.0.1:8545/".to_string());

        let target_address = matches
            .get_one::<String>("address")
            .ok_or_else(|| "Ethereum address is required".to_string())
            .and_then(|s| {
                Address::from_str(s)
                    .map_err(|_| "Invalid Ethereum address format".to_string())
            })?;

        Ok(Config {
            ws_url,
            target_address,
        })
    }
}
