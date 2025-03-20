#[cfg(test)]
mod tests {
    use csv::{Writer, WriterBuilder};
    use rain_out::api::ChainResult;
    use rain_out::error::Error;
    use std::fs::File;
    use std::path::Path;

    #[tokio::test]
    async fn csv_test() -> Result<(), Error> {
        let output_path = Path::new(".");
        let filen = File::create(output_path.join("blocks.csv"))?;
        // let mut blocks_writer = Writer::from_writer(filen);

        let mut wtr = WriterBuilder::new().has_headers(false).from_writer(filen);

        let summary: ChainResult = ChainResult {
            total_tx: 0u64,
            failed_tx: 0u64,
            successful: 0u64,
            txs: Vec::new(),
            network: rain_out::cli::Networks::Flare,
        };
        wtr.serialize(summary)?;

        Ok(())
    }
}
