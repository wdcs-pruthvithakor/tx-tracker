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
            let mut sub = match web3.eth_subscribe().subscribe_new_heads().await {
                Ok(sub) => sub,
                Err(e) => {
                    eprintln!("Error subscribing to new blocks: {:?}. Retrying...", e);
                    sleep(Duration::from_secs(5)).await;
                    continue;
                }
            };

            // Start listening to new transactions
            println!("Listening for transactions to {:?}", self.target_address);

            // Process incoming blocks
            while let Some(block_header) = sub.next().await {
                match block_header {
                    Ok(block_header) => {
                        // Fetch block details
                        match web3.eth().block_with_txs(web3::types::BlockId::Hash(block_header.hash.unwrap_or_default())).await {
                            Ok(Some(block)) => {
                                // Iterate through transactions in the block
                                for tx in block.transactions {
                                    // Handle `from` and `to` addresses
                                    let from_address = match tx.from {
                                        Some(from) => from,
                                        None => {
                                            eprintln!("Missing 'from' address in transaction: {:?}", tx.hash);
                                            continue;
                                        }
                                    };

                                    let to_address = match tx.to {
                                        Some(to) => to,
                                        None => {
                                            eprintln!("Missing 'to' address in transaction: {:?}", tx.hash);
                                            continue;
                                        }
                                    };

                                    // If the transaction is to the target address
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
                            }
                            Ok(None) => eprintln!("Transaction not found for block hash: {:?}", block_header.hash.unwrap_or_default()),
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
