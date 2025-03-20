use clap::Parser;
use log::{info, warn};
use rain_out::api::handle_input;
use rain_out::{cli, error::Error};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let args = cli::Args::parse();
    info!("Network: {:?}", args.network);
    info!("Contract Address: {}", args.orderbook_address);
    info!(
        "Block Range: {} to {}",
        args.block_range[0], args.block_range[1]
    );
    info!("Downtime Theshold: {} blocks", args.downtime_threshold);
    let _output = match handle_input(
        args.network,
        args.block_range[0],
        args.block_range[1],
        args.orderbook_address.as_str(),
        args.downtime_threshold,
        Path::new("."),
    )
    .await
    {
        Ok(result) => result,
        Err(e) => {
            warn!("Error encountered: {:?}", e);
            return Err(e);
        }
    };

    Ok(())
}
