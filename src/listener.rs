use futures::stream::StreamExt;
use web3::transports::WebSocket;
use web3::types::{Address, U256};
use web3::Web3;
// use web3::types::FilterBuilder;
use crate::config::Config;

pub struct Listener {
    web3: Web3<WebSocket>,
    target_address: Address,
}

impl<'a> Listener {
    pub async fn new(config: &'a Config) -> web3::Result<Self> {
        // The WebSocket creation requires `.await` as it's asynchronous
        let transport = WebSocket::new(&config.ws_url).await?;
        let web3 = Web3::new(transport);

        Ok(Listener {
            web3,
            target_address: config.target_address,
        })
    }

    pub async fn listen(&self) -> web3::Result<()> {
        // Create a filter to listen to transactions to the target address
        // let _filter = FilterBuilder::default()
        //     .address(vec![self.target_address])
        //     .build();

        let mut sub = self.web3.eth_subscribe().subscribe_new_pending_transactions().await?;

        while let Some(tx_hash) = sub.next().await {
            if let Ok(tx_hash) = tx_hash {
                // Fetch transaction details
                if let Ok(Some(tx)) = self.web3.eth().transaction(web3::types::TransactionId::Hash(tx_hash)).await {
                    if let Some(to_address) = tx.to {
                        if to_address == self.target_address {
                            println!(
                                "New Transaction Received!\nTx Hash: {:?}\nFrom: {:?}\nTo: {:?}\nValue: {:?} ETH\n",
                                tx.hash,
                                tx.from,
                                to_address,
                                tx.value
                            );
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
