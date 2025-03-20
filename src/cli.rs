use crate::api::validate_eth_address;
use clap::{Parser, ValueEnum};
use serde::Serialize;
use std::fmt;

//Network enum
#[derive(Debug, Clone, ValueEnum, Serialize)]
#[value(rename_all = "lower")]
pub enum Networks {
    Ethereum,
    Flare,
    Base,
    Polygon,
    Arbitrum,
    Linea,
}

// print network
impl fmt::Display for Networks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Networks::Ethereum => "Ethereum",
            Networks::Flare => "Flare",
            Networks::Base => "Base",
            Networks::Polygon => "Polygon",
            Networks::Arbitrum => "Arbitrum",
            Networks::Linea => "Linea",
        };
        write!(f, "{}", s)
    }
}

//check if blocknr is higher than latest blocknr

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about,
 //   example = "./target/release/rainuptimetrack --network flare --orderbook-address 0x0deadbeef --downtime-threshold 10 --block-range 100,200",
 long_about = None)]
pub struct Args {
    // Select network
    #[arg(short, long)]
    pub network: Networks,

    /// Orderbook eth contract address, validate ethereum address
    #[arg(short, long)] //, value_parser = validate_eth_address
    pub orderbook_address: String,

    /// Block range in format START,END
    #[arg(short, long, value_delimiter = ',', num_args = 2)]
    pub block_range: Vec<u64>,

    /// Downtime threshold in blocks
    #[arg(short, long, value_parser = clap::value_parser!(u64).range(1..90000))]
    pub downtime_threshold: u64,
}

/// Custom validation error type
#[derive(Debug)]
pub struct ValidationError(String);

impl std::error::Error for ValidationError {}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Validation error: {}", self.0)
    }
}
