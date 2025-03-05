use anyhow::Result;
use clap::Parser;
use rainuptimetrack::api::handle_input;
use rainuptimetrack::cli;

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::Args::parse(); // todo validate input
    println!("Network: {:?}", args.network);
    println!("Orderbook Address: {}", args.orderbook_address);
    println!(
        "Block Range: {} to {}",
        args.block_range[0], args.block_range[1]
    );
    println!("Downtime Threeshold: {}", args.downtime_threshold);
    let output = match handle_input(
        args.network,
        args.block_range[0],
        args.block_range[1],
        args.orderbook_address.as_str(),
    )
    .await
    {
        Ok(result) => result,
        Err(_e) => return Err(anyhow::anyhow!("could not query")),
    };
    println!(
        "
Query result:

Failed tx:      {:?}
Successfull tx: {:?}
Total tx:       {:?}
    ",
        output.failed_tx, output.successful, output.total_tx
    );
    println!("__finished__");
    Ok(())
}
