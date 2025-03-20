use crate::cli::Networks;
use crate::error::Error;
use crate::rpc::ChainClient;
use csv::WriterBuilder;
use hypersync_client::{QueryResponse, simple_types::Transaction};
use sha3::{Digest, Keccak256};
//use hypersync_format::Hex;
use hypersync_format::TransactionStatus;
use log::{debug, info, warn};
use serde::Serialize;
use std::fs::File;

/// Result summary of query
#[derive(Debug, Serialize)]
pub struct ChainResult {
    /// total amount of transactions
    pub total_tx: u64,
    /// total amount of failed
    pub failed_tx: u64,
    /// amount of successful transactions
    pub successful: u64,
    /// list of transactions
    pub txs: Vec<u8>,
    /// Network
    pub network: Networks,
}

pub struct OutputResults {
    pub blocknr: u64,
    pub field_type: String,
}

///  validate ethereum address
///  based on the ts version:
///  https://github.com/web3/web3.js/blob/bf1691765bd9d4b0f7a4479e915207707d69226d/packages/web3-validator/src/validation/address.ts#L50
pub fn validate_eth_address(eth_address: &str) -> Result<bool, Error> {
    // remove prefix 0x if it has it
    let filterd_address = match eth_address.strip_prefix("0x") {
        Some(value) => value, // if it starts with 0x, remove 0x and return new
        None => eth_address,  // else return original
    };
    // check that it contains legit ascii characters only and that length is 40
    if !filterd_address.chars().all(|c| c.is_ascii_hexdigit()) && filterd_address.len() == 40 {
        return Err(Error::InvalidAddressFormat);
    }

    // address Checksum validation
    let address_lower = filterd_address;
    let hash = Keccak256::digest(address_lower.as_bytes());
    let hash_hex = hex::encode(hash);

    address_lower.chars().enumerate().all(|(i, c)| {
        let hash_byte = hash_hex.as_bytes()[i];
        let nibble = if i % 2 == 0 {
            hash_byte >> 4
        } else {
            hash_byte & 0x0f
        };

        match nibble >= 8 {
            true => c.is_ascii_uppercase(),
            false => c.is_ascii_lowercase(),
        }
    });

    Ok(true)
}

pub fn summerize_result(transactions: Vec<Transaction>, network: Networks) -> ChainResult {
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
        txs: Vec::new(), //transactions
        network,
    }
}

/// handle input from cli and output query result
pub async fn handle_input(
    network: Networks,
    block_start: u64,
    block_stop: u64,
    contract_address: &str,
    block_limit: u64,
    output_path: &std::path::Path,
) -> Result<bool, Error> {
    // get a rpc client
    let client = ChainClient::get_network_client(network.clone())?;

    // return a list of responses
    let query: Vec<QueryResponse> = client
        .query_chain(block_start, block_stop, contract_address)
        .await?;

    // Calculate total capacity of amount of txn
    let total_txs: &usize = &query
        .iter()
        .flat_map(|res| &res.data.transactions)
        .map(|batch| batch.len())
        .sum();

    // set a vec with the tx amount
    let mut loot = Vec::with_capacity(total_txs.to_owned());
    for response in &query {
        // add all txns
        loot.extend(response.to_owned().data.transactions.into_iter().flatten());
    }

    let mut blocks: Vec<u64> = Vec::with_capacity(estimate_capacity(&query)); // dont allocate more than needed 

    // Initialize CSV writers
    let filen = File::create(output_path.join("blocks.csv"))?;
    //let mut  = Writer::from_writer(filen);

    let mut blocks_writer = WriterBuilder::new().has_headers(false).from_writer(filen);

    for entry in &query {
        for log_batch in &entry.data.logs {
            for log in log_batch {
                let block_number = log
                    .block_number
                    .ok_or(Error::MissingField(String::from("block_number")))?
                    .try_into()
                    .map_err(|_| {
                        warn!("could not convert block nr");
                        Error::Conversion(String::from("block number"))
                    })?;
                blocks.push(block_number);
            }
        }
    }

    let summary = summerize_result(loot, network);

    blocks.sort();

    for i in 1..blocks.len() {
        let gap = blocks[i] - blocks[i - 1];
        if gap > block_limit {
            info!(
                "[Threshold detected]Large gap between txs in {} and {}: {} blocks",
                blocks[i - 1],
                blocks[i],
                gap
            );
        }
    }

    info!("Total tx:       {:?}", blocks.len());

    debug!("writing csv file");
    // Write block record to CSV
    blocks_writer.serialize(summary)?;

    debug!("csv file ok");
    Ok(true)
}

/// estimate query response size
fn estimate_capacity(query: &[QueryResponse]) -> usize {
    query
        .iter()
        .flat_map(|r| &r.data.logs)
        .map(|b| b.len())
        .sum()
}
