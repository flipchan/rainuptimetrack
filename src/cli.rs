use clap::{Parser, ValueEnum};

//Network enum
#[derive(Debug, Clone, ValueEnum)]
#[value(rename_all = "lower")]
pub enum Networks {
    Ethereum,
    Flare,
    Base,
    Polygon,
    Arbitrum,
    Linea,
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

    /// Orderbook contract address
    #[arg(short, long)]
    pub orderbook_address: String,

    /// Block range in format START,END
    #[arg(short, long, value_delimiter = ',', num_args = 2)]
    pub block_range: Vec<u64>,

    /// Downtime threshold in blocks
    #[arg(short, long)]
    pub downtime_threshold: u64,
}
