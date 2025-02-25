use futures::stream::StreamExt;
use web3::transports::WebSocket;
use web3::types::{Address, U256};
use web3::Web3;
use crate::config::Config;

pub struct Listener {
    web3: Web3<WebSocket>,
    target_address: Address,
}

impl Listener {
    pub async fn new(config: &Config) -> web3::Result<Self> {
        let transport = WebSocket::new(&config.ws_url).await?;
        let web3 = Web3::new(transport);

        Ok(Listener {
            web3,
            target_address: config.target_address,
        })
    }

    pub async fn listen(&self) -> web3::Result<()> {
        let mut sub = self.web3.eth_subscribe().subscribe_new_pending_transactions().await?;

        while let Some(tx_hash) = sub.next().await {
            match tx_hash {
                Ok(tx_hash) => {
                    // Fetch transaction details
                    match self.web3.eth().transaction(web3::types::TransactionId::Hash(tx_hash)).await {
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

        Ok(())
    }

    fn convert_wei_to_eth(&self, wei: U256) -> f64 {
        // Convert Wei (U256) to Ether (f64)
        let wei_str = wei.to_string();
        let wei_value: f64 = wei_str.parse().unwrap_or(0.0); // convert Wei to f64
        let ether_value = wei_value / 1e18; // 1 ETH = 10^18 Wei
        ether_value
    }
}
