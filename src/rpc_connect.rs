//e hypersync_client::{Client, ClientConfig};
use crate::chains::{ETH_RPC, FLARE_RPC, LINEA_RPC, POLYGON_RPC};
use hypersync_client::QueryResponse;
use hypersync_client::{
    net_types::Query, simple_types::Transaction, CallDecoder, Client, ClientConfig,
};
use tokio::time::{sleep, Duration};
//use hyperfuel_client::{Client, ClientConfig};
//use std::num::NonZeroU64;
use std::sync::Arc;
//use url::Url;

/// Chain Connection client
pub struct ChainClient {
    client: Client,
}

impl ChainClient {
    pub fn evm_client(rpc_url: &str) -> Client {
        let client = Client::new(ClientConfig {
            url: Some(rpc_url.parse().unwrap()),
            ..Default::default()
        })
        .unwrap(); // todo handle errors and wrap in result
        client
    }

    /// flare hypersync client
    pub fn flare_client() -> Self {
        Self {
            client: Self::evm_client(FLARE_RPC),
        }
    }

    /// polygon hypersync client
    pub fn polygon_client() -> Self {
        Self {
            client: Self::evm_client(POLYGON_RPC),
        }
    }

    /// ethereum hypersync client
    pub fn ethereum_client() -> Self {
        Self {
            client: Self::evm_client(ETH_RPC),
        }
    }
    /// return the latest block / current block height
    pub async fn latest_block(&self) -> u64 {
        // return latest height if not return 0
        match self.client.get_height().await {
            Ok(value) => value,
            _ => 0u64,
        }
    }

    /// linea hypersync client
    pub fn linea_client() -> Self {
        Self {
            client: Self::evm_client(LINEA_RPC),
        }
    }

    /// query a chain between a blocklimit for all tx to a contract
    pub async fn query_chain(
        &self,
        block_start: u64,
        block_stop: u64,
        contract_address: &str,
    ) -> Vec<Transaction> {
        let clientme: Arc<Client> = Arc::new(self.client.clone()); // move into arc

        let mut q: Query = serde_json::from_value(serde_json::json!( {
                        "from_block": block_start,
                        "to_block": block_stop,
        // query for transactions going to our contract
                        "transactions": [
                            {
                                "to": [contract_address],
                            },
                        ],

                        "field_selection": {
                            "log": [
                                "address",
                                "data",
                                "number",
                                "timestamp",
                                "input",
                                "to",
                                "status",
                                "from",
                            ],
                            "transaction": [
                                "hash",
                                "input",
                                "status",
                                "number",
                                "transaction_index",
                                "from",
                                "to",
                                "value",
                            ],
                        }
                    }))
        .unwrap();
        let mut loot: Vec<Transaction> = Vec::new();

        let decoder: CallDecoder = CallDecoder::from_signatures(&[
            "arb3(address orderBook,tuple takeOrders,tuple task)",
            "transferFrom(address src, address dst, uint256 wad)",
        ])
        .unwrap();

        loop {
            let res: QueryResponse = clientme.get(&q).await.unwrap();

            for batch in res.data.transactions {
                // batch is : Vec<Transaction>
                //loot.append(&batch);
                for tx in batch {
                    //           println!("Found tx");

                    loot.push(tx.clone());
                    //    let dec = tx.clone().status.unwrap();
                    //     println!("Tx status: {:?}", dec);
                    //       let blocknr = tx.clone();
                    //     println!("Raw TX: {:?}", blocknr.input.unwrap().to_vec());
                    //    println!("Decoded value: {:?}", dec);
                    if let Some(decoded_call) = decoder.decode_input(&tx.input.unwrap()).unwrap() {
                        println!("yum yum");
                        if decoded_call.len() == 2 {
                            println!(
                                "Found arb3 transfer {:?}. to: {}, amount: {:?}",
                                &tx.hash,
                                decoded_call[0].as_address().unwrap(),
                                decoded_call[1].as_uint().unwrap()
                            );
                        } else if decoded_call.len() == 3 {
                            println!(
                                "Found DAU transfer {:?}. from: {}, to: {}, amount: {:?}",
                                &tx.hash,
                                decoded_call[0].as_address().unwrap(),
                                decoded_call[1].as_address().unwrap(),
                                decoded_call[2].as_uint().unwrap()
                            );
                        }
                    }
                }
            }
            println!("scanned up to block {}", res.next_block);
            if res.next_block >= block_stop {
                break;
            }

            if let Some(archive_height) = res.archive_height {
                if archive_height < res.next_block {
                    // wait if we are at the head
                    // notice we use explicit get_height in order to not waste data requests.
                    // get_height is lighter compared to spamming data requests at the tip.
                    while clientme.get_height().await.unwrap() < res.next_block {
                        sleep(Duration::from_secs(1)).await;
                    }
                }
            }

            // continue query from next_block
            q.from_block = res.next_block;
        }

        loot
    }
}
