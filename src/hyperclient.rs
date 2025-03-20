use crate::error::Error;
use hypersync_client::net_types::Query;

/// build a hypersync query
pub fn build_query(
    block_start: u64,
    block_stop: u64,
    contract_address: &str,
) -> Result<Query, Error> {
    let q: Query = serde_json::from_value(serde_json::json!( {
            "from_block": block_start,
                   "to_block": block_stop,
    // query for transactions going to our contract
       "logs": [
    {
    "address": [
        contract_address
    ]
    }
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
                    "block_number",
                    "block_hash",
                    "value",
                    "contract_address",
                    "topic0",
                    "topic1",
                    "output",
                    "topic2"
                ],
                "transaction": [
                    "hash",
                    "input",
                    "value",
                    "block_hash",
                    "status",
                    "number",
                    "block_number",
                    "transaction_index",
                    "from",
                    "output",
                    "contract_address",
                    "to",
                    "value",
                    "topic0",
                    "topic1",
                    "topic2"
                ],
            }
        }))
    .unwrap();

    Ok(q)
}
