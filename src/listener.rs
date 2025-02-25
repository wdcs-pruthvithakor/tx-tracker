use futures::stream::StreamExt;
use web3::transports::WebSocket;
use web3::types::{Address, U256};
use web3::Web3;
use crate::config::Config;
use tokio::time::{sleep, Duration};

pub struct Listener {
    ws_url: String,
    target_address: Address,
}

impl Listener {
    pub fn new(config: &Config) -> Self {
        Listener {
            ws_url: config.ws_url.clone(),
            target_address: config.target_address,
        }
    }

    pub async fn listen(&self) -> web3::Result<()> {
        loop {
            let transport = match WebSocket::new(&self.ws_url).await {
                Ok(transport) => transport,
                Err(e) => {
                    eprintln!("Error establishing WebSocket connection: {:?}. Retrying...", e);
                    sleep(Duration::from_secs(5)).await;
                    continue;
                }
            };
            
            let web3 = Web3::new(transport);
            let mut sub = match web3.eth_subscribe().subscribe_new_pending_transactions().await {
                Ok(sub) => sub,
                Err(e) => {
                    eprintln!("Error subscribing to new transactions: {:?}. Retrying...", e);
                    sleep(Duration::from_secs(5)).await;
                    continue;
                }
            };

            // Start listening to new transactions
            println!("Listening for transactions to {:?}", self.target_address);

            // Process incoming transactions
            while let Some(tx_hash) = sub.next().await {
                match tx_hash {
                    Ok(tx_hash) => {
                        // Fetch transaction details
                        match web3.eth().transaction(web3::types::TransactionId::Hash(tx_hash)).await {
                            Ok(Some(tx)) => {
                                // Handle `from` and `to` addresses
                                let from_address = match tx.from {
                                    Some(from) => from,
                                    None => {
                                        eprintln!("Missing 'from' address in transaction: {:?}", tx_hash);
                                        continue;
                                    }
                                };

                                let to_address = match tx.to {
                                    Some(to) => to,
                                    None => {
                                        eprintln!("Missing 'to' address in transaction: {:?}", tx_hash);
                                        continue;
                                    }
                                };

                                if to_address == self.target_address {
                                    let value_in_eth = self.convert_wei_to_eth(tx.value);
                                    println!(
                                        "New Transaction Received!\nTx Hash: {:?}\nFrom: {:?}\nTo: {:?}\nValue: {:?} ETH\n",
                                        tx.hash,
                                        from_address,
                                        to_address,
                                        value_in_eth
                                    );
                                }
                            }
                            Ok(None) => eprintln!("Transaction not found for hash: {:?}", tx_hash),
                            Err(e) => eprintln!("Error fetching transaction details: {:?}", e),
                        }
                    }
                    Err(e) => eprintln!("Error receiving transaction hash: {:?}", e),
                }
            }

            // If the subscription ends, retry the whole process
            eprintln!("WebSocket connection closed. Reconnecting...");
            sleep(Duration::from_secs(5)).await;
        }
    }

    fn convert_wei_to_eth(&self, wei: U256) -> f64 {
        // Convert Wei (U256) to Ether (f64)
        let wei_str = wei.to_string();
        let wei_value: f64 = wei_str.parse().unwrap_or(0.0); // convert Wei to f64
        let ether_value = wei_value / 1e18; // 1 ETH = 10^18 Wei
        ether_value
    }
}
