use crate::cli::Networks;
use crate::rpc_connect::ChainClient;
use anyhow::Error;
use hypersync_client::simple_types::{Log, Transaction};
use hypersync_format::Hex;
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

pub struct OutputResults {
    pub blocknr: u64,
    pub field_type: String,
}

#[allow(dead_code)] // will be used later
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
/// todo filter based on events
pub async fn handle_input(
    network: Networks,
    block_start: u64,
    block_stop: u64,
    contract_address: &str,
    block_limit: u64,
) -> Result<bool, Error> {
    let mut client = ChainClient::ethereum_client(); // set a default client
    match network {
        // select network and tell client
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

    // return a list of responses
    let query = client
        .query_chain2(block_start, block_stop, contract_address)
        .await;

    let mut local: Vec<Log> = Vec::new();
    let mut resulto: Vec<OutputResults> = Vec::new(); //

    for entry in &query {
        // Iterate through each log batch in the page
        for log_batch in &entry.data.logs {
            // Iterate through individual logs in the batch
            for log in log_batch {
                local.push(log.clone());
                let block_nr = log.block_number.unwrap().encode_hex();
                let br = block_nr.trim_start_matches("0x");
                match u64::from_str_radix(br, 16) {
                    Ok(num) => {
                        //        println!("Decoded value: {}", num);
                        resulto.push(OutputResults {
                            blocknr: num,
                            field_type: log.topics.first().unwrap().clone().unwrap().encode_hex(),
                        });
                    } // 26
                    Err(e) => println!("Error: {}", e),
                }
            }
        }
    }

    let mut blocks: Vec<u64> = resulto.iter().map(|r| r.blocknr).collect();

    blocks.sort();

    for i in 1..blocks.len() {
        let gap = blocks[i] - blocks[i - 1];
        if gap > block_limit {
            println!(
                "[Threshold detected]Large gap between txs in {} and {}: {} blocks",
                blocks[i - 1],
                blocks[i],
                gap
            );
        }
    }

    println!("Total tx:       {:?}", blocks.len());

    Ok(true)
}
