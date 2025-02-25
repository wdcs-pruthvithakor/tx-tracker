use clap::{Command, Arg};
use web3::types::Address;
use std::str::FromStr;

pub struct Config {
    pub ws_url: String,
    pub target_address: Address,
}

impl Config {
    pub fn from_args() -> Self {
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

        let ws_url = matches.get_one::<String>("url").unwrap().to_string();
        let target_address = Address::from_str(matches.get_one::<String>("address").unwrap())
            .expect("Invalid Ethereum address format");

        Config {
            ws_url,
            target_address,
        }
    }
}
