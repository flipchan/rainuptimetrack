// Fuzzable transaction types

use arbitrary::Arbitrary;
use hypersync_format::Quantity;
use hypersync_format::TransactionType;
use hypersync_format::{
    AccessList, Address, BlockNumber, BloomFilter, Data, Hash, TransactionIndex, TransactionStatus,
};
use serde::Serialize;

#[derive(arbitrary::Arbitrary, Debug, Serialize)]
pub struct FixedSizeData(Box<[u8; 32]>);

/// modify ChainResult to make it fuzz friendly
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct ChainResultFuzz {
    pub total_tx: u64,
    pub failed_tx: u64,
    pub successful: u64,
    #[arbitrary(with = arbitrary_transactions)]
    pub txs: Vec<FuzzTransaction>,
    pub network: Networks,
}

fn arbitrary_transactions(
    u: &mut arbitrary::Unstructured<'_>,
) -> arbitrary::Result<Vec<FuzzTransaction>> {
    let count = u.int_in_range(0..=10)?;
    (0..count).map(|_| FuzzTransaction::arbitrary(u)).collect()
}

//Network enum
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum Networks {
    Ethereum,
    Flare,
    Base,
    Polygon,
    Arbitrum,
    Linea,
}

// from
#[derive(arbitrary::Arbitrary, Debug, Serialize)]
pub struct FuzzTransaction {
    pub block_hash: Option<FixedSizeData>,
    pub block_number: Option<u64>,
    pub from: Option<[u8; 20]>,
    pub gas: Option<u128>,
    pub gas_price: Option<u128>,
    pub hash: Option<FixedSizeData>,
    pub input: Option<Vec<u8>>,
    pub nonce: Option<u64>,
    pub to: Option<[u8; 20]>,
    pub transaction_index: Option<u64>,
    pub value: Option<u128>,
    pub v: Option<u64>,
    pub r: Option<u128>,
    pub s: Option<u128>,
    pub y_parity: Option<u64>,
    pub max_priority_fee_per_gas: Option<u128>,
    pub max_fee_per_gas: Option<u128>,
    pub chain_id: Option<u64>,
    pub access_list: Option<Vec<FuzzAccessList>>,
    pub max_fee_per_blob_gas: Option<u128>,
    pub blob_versioned_hashes: Option<Vec<FixedSizeData>>,
    pub cumulative_gas_used: Option<u128>,
    pub effective_gas_price: Option<u128>,
    pub gas_used: Option<u128>,
    pub contract_address: Option<[u8; 20]>,
    pub logs_bloom: Option<Vec<u8>>,
    pub kind: Option<u8>,
    pub root: Option<FixedSizeData>,
    pub status: Option<bool>,
    pub l1_fee: Option<u128>,
    pub l1_gas_price: Option<u128>,
    pub l1_gas_used: Option<u128>,
    pub l1_fee_scalar: Option<f64>,
    pub gas_used_for_l1: Option<u128>,
}

#[derive(arbitrary::Arbitrary, Debug, Serialize)]
pub struct FuzzAccessList {
    pub address: [u8; 20],
    pub storage_keys: Vec<FixedSizeData>,
}

fn arbitrary_bytes(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Vec<u8>> {
    let size = u.int_in_range(0..=32)?;
    Ok(u.bytes(size)?.to_vec())
}

#[derive(arbitrary::Arbitrary, Debug, Serialize)]
pub struct FuzzHash(#[arbitrary(with = arbitrary_bytes)] pub Vec<u8>);

/// wrapper to support hypersync_client::simple_types::Transaction
impl From<FuzzTransaction> for hypersync_client::simple_types::Transaction {
    fn from(fuzz: FuzzTransaction) -> Self {
        hypersync_client::simple_types::Transaction {
            block_hash: fuzz.block_hash.map(|h| Hash::from(h)),
            block_number: fuzz.block_number.map(|h| BlockNumber::from(h)),
            from: fuzz.from.map(Address::from),
            gas: fuzz.gas.map(|q| Quantity::from(q.to_be_bytes().to_vec())),
            gas_price: fuzz
                .gas_price
                .map(|q| Quantity::from(q.to_be_bytes().to_vec())),
            hash: fuzz.hash.map(|h| Hash::from(h)),
            input: fuzz.input.map(Data::from),
            nonce: fuzz.nonce.map(|n| Quantity::from(n.to_be_bytes().to_vec())),
            to: fuzz.to.map(Address::from),
            transaction_index: fuzz.transaction_index.map(|v| TransactionIndex::from(v)),
            value: fuzz.value.map(|q| Quantity::from(q.to_be_bytes().to_vec())),
            v: fuzz.v.map(|v| Quantity::from(v.to_be_bytes().to_vec())),
            r: fuzz.r.map(|r| Quantity::from(r.to_be_bytes().to_vec())),
            s: fuzz.s.map(|s| Quantity::from(s.to_be_bytes().to_vec())),
            y_parity: fuzz
                .y_parity
                .map(|y| Quantity::from(y.to_be_bytes().to_vec())),
            max_priority_fee_per_gas: fuzz
                .max_priority_fee_per_gas
                .map(|q| Quantity::from(q.to_be_bytes().to_vec())),

            max_fee_per_gas: fuzz
                .max_fee_per_gas
                .map(|q| Quantity::from(q.to_be_bytes().to_vec())),
            chain_id: fuzz
                .chain_id
                .map(|c| Quantity::from(c.to_be_bytes().to_vec())),
            access_list: fuzz.access_list.map(|list| {
                list.into_iter()
                    .map(|al| AccessList {
                        address: Some(Address::from(al.address)),
                        storage_keys: Some(
                            al.storage_keys
                                .into_iter()
                                .map(|sk| Hash::from(sk.0))
                                .collect::<Vec<_>>(),
                        ), // force vec
                    })
                    .collect::<Vec<_>>() // Collect  Vec
            }),
            max_fee_per_blob_gas: fuzz
                .max_fee_per_blob_gas
                .map(|q| Quantity::from(q.to_be_bytes().to_vec())),
            blob_versioned_hashes: fuzz
                .blob_versioned_hashes
                .map(|hashes| hashes.into_iter().map(|h| Hash::from(h)).collect()),
            cumulative_gas_used: fuzz
                .cumulative_gas_used
                .map(|q| Quantity::from(q.to_be_bytes().to_vec())),
            effective_gas_price: fuzz
                .effective_gas_price
                .map(|q| Quantity::from(q.to_be_bytes().to_vec())),
            gas_used: fuzz
                .gas_used
                .map(|q| Quantity::from(q.to_be_bytes().to_vec())),
            contract_address: fuzz.contract_address.map(Address::from),
            logs_bloom: fuzz.logs_bloom.map(BloomFilter::from),
            kind: fuzz.kind.and_then(|k| TransactionType::try_from(k).ok()),
            root: fuzz.root.map(|h| Hash::from(h)),
            status: fuzz.status.map(|s| {
                if s {
                    TransactionStatus::Success
                } else {
                    TransactionStatus::Failure
                }
            }),
            l1_fee: fuzz
                .l1_fee
                .map(|q| Quantity::from(q.to_be_bytes().to_vec())),
            l1_gas_price: fuzz
                .l1_gas_price
                .map(|q| Quantity::from(q.to_be_bytes().to_vec())),
            l1_gas_used: fuzz
                .l1_gas_used
                .map(|q| Quantity::from(q.to_be_bytes().to_vec())),
            l1_fee_scalar: fuzz.l1_fee_scalar,
            gas_used_for_l1: fuzz
                .gas_used_for_l1
                .map(|q| Quantity::from(q.to_be_bytes().to_vec())),
        }
    }
}

/*
impl From<FixedSizeData> for hypersync_format::FixedSizeData<32> { // use static 32 FixedSizeData for fuzz testing
    fn from(bytes: FixedSizeData) -> Self {
        hypersync_format::FixedSizeData::<32>::from(bytes)
    }
}
*/

impl From<FixedSizeData> for hypersync_format::FixedSizeData<32> {
    fn from(fuzz_data: FixedSizeData) -> Self {
        // Access the inner [u8; 32] and convert directly
        hypersync_format::FixedSizeData::from(fuzz_data.0)
    }
}
