use crate::cli::Networks;
use crate::rpc_connect::ChainClient;
use anyhow::Error;
use hypersync_client::simple_types::Transaction;
use hypersync_format::TransactionStatus;

/// Result summary of query
pub struct ChainResult {
    /// total amount of transactions
    pub total_tx: u64,
    /// total amount of failed
    pub failed_tx: u64,
    ///
    pub successful: u64,
    /// list of transactions
    pub txs: Vec<Transaction>,
    /// Network
    pub network: Networks,
}

fn summerize_result(transactions: Vec<Transaction>, network: Networks) -> ChainResult {
    let successfulltx: Vec<Transaction> = transactions
        .iter()
        .filter(|tx| tx.status == Some(TransactionStatus::Success))
        .cloned() // Convert &Transaction to Transaction
        .collect();
    let failedtx: Vec<Transaction> = transactions
        .iter()
        .filter(|tx| tx.status == Some(TransactionStatus::Failure))
        .cloned() // Convert &Transaction to Transaction
        .collect();

    ChainResult {
        total_tx: transactions.len() as u64,
        failed_tx: failedtx.len() as u64,
        successful: successfulltx.len() as u64,
        txs: transactions,
        network: network,
    }
}

/// handle input from cli and output query result
pub async fn handle_input(
    network: Networks,
    block_start: u64,
    block_stop: u64,
    contract_address: &str,
) -> Result<ChainResult, Error> {
    let mut client = ChainClient::ethereum_client(); // set a default client
    match network {
        Networks::Flare => {
            client = ChainClient::flare_client();
        }
        Networks::Polygon => {
            client = ChainClient::polygon_client();
        }
        Networks::Ethereum => {}
        Networks::Linea => {
            client = ChainClient::linea_client();
        }

        _ => {
            return Err(anyhow::anyhow!("Network not currently supported"));
        }
    }

    let query = client
        .query_chain(block_start, block_stop, contract_address)
        .await;
    let result = summerize_result(query, network);
    Ok(result)
}
