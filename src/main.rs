use anyhow::Result;
use clap::Parser;
use rainuptimetrack::api::handle_input;
use rainuptimetrack::cli;

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::Args::parse(); // todo validate input
    println!("Network: {:?}", args.network);
    println!("Contract Address: {}", args.orderbook_address);
    println!(
        "Block Range: {} to {}",
        args.block_range[0], args.block_range[1]
    );
    println!("Downtime Theshold: {} blocks", args.downtime_threshold);
    let _output = match handle_input(
        args.network,
        args.block_range[0],
        args.block_range[1],
        args.orderbook_address.as_str(),
        args.downtime_threshold,
    )
    .await
    {
        Ok(result) => result,
        Err(_e) => return Err(anyhow::anyhow!("could not query")),
    };

    println!("__finished__");
    Ok(())
}
