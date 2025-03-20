use crate::chains::{ETH_RPC, FLARE_RPC, LINEA_RPC, POLYGON_RPC};
use crate::cli::Networks;
use crate::error::Error;
use crate::hyperclient::build_query;
use hypersync_client::QueryResponse;
use hypersync_client::{Client, ClientConfig, net_types::Query};
use log::{debug, info};
use std::sync::Arc;
use tokio::time::{Duration, sleep};

/// Chain Connection client
pub struct ChainClient {
    client: Client,
}

impl ChainClient {
    /// get a rpc network client
    pub fn get_network_client(network: Networks) -> Result<Self, Error> {
        match network {
            // select network and return client
            Networks::Flare => Self::flare_client(),
            Networks::Polygon => Self::polygon_client(),
            Networks::Ethereum => Self::ethereum_client(),
            Networks::Linea => Self::linea_client(),

            _ => Err(Error::InvalidNetwork),
        }
    }

    /// return a evm rpc client
    pub fn evm_client(rpc_url: &str) -> Result<Client, Error> {
        Ok(Client::new(ClientConfig {
            url: Some(rpc_url.parse()?),
            ..Default::default()
        })?)
    }

    /// flare hypersync client
    pub fn flare_client() -> Result<Self, Error> {
        Ok(Self {
            client: Self::evm_client(FLARE_RPC)?,
        })
    }

    /// polygon hypersync client
    pub fn polygon_client() -> Result<Self, Error> {
        Ok(Self {
            client: Self::evm_client(POLYGON_RPC)?,
        })
    }

    /// ethereum hypersync client
    pub fn ethereum_client() -> Result<Self, Error> {
        Ok(Self {
            client: Self::evm_client(ETH_RPC)?,
        })
    }

    /// linea hypersync client
    pub fn linea_client() -> Result<Self, Error> {
        Ok(Self {
            client: Self::evm_client(LINEA_RPC)?,
        })
    }

    /// return the latest block / current block height
    pub async fn latest_block(&self) -> u64 {
        // return latest height if not return 0
        match self.client.get_height().await {
            Ok(value) => value,
            _ => 0u64,
        }
    }

    /// query a chain between a blocklimit for all tx to a contract
    /// use this
    pub async fn query_chain(
        &self,
        block_start: u64,
        block_stop: u64,
        contract_address: &str,
    ) -> Result<Vec<QueryResponse>, Error> {
        let clientme: Arc<Client> = Arc::new(self.client.clone()); // move into arc

        let mut q: Query = match build_query(block_start, block_stop, contract_address) {
            Ok(query) => query,
            Err(_e) => return Err(Error::QueryError),
        };
        let mut loot: Vec<QueryResponse> = Vec::new();

        loop {
            let res: QueryResponse = clientme.get(&q).await?;
            loot.push(res.clone());

            info!("scanned up to block {}", res.next_block);
            if res.next_block >= block_stop {
                // if the block limit is hit, break
                break;
            }

            if let Some(archive_height) = res.archive_height {
                self.wait_for_block(clientme.clone(), res.next_block, archive_height)
                    .await?;
            }

            // continue query from next_block
            q.from_block = res.next_block;
        }

        Ok(loot)
    }

    /// wait for the next block
    async fn wait_for_block(
        &self,
        client: Arc<Client>,
        target_block: u64,
        current_archive_height: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        const RETRY_DELAY: Duration = Duration::from_secs(7);

        if current_archive_height < target_block {
            while client.get_height().await? < target_block {
                debug!("Waiting for block: #{}", target_block);
                sleep(RETRY_DELAY).await; // chill for 7sec
            }
        }
        Ok(())
    }
}
