#[cfg(test)]
mod tests {
    use hypersync_client::simple_types::Log;
    //   use super::*;
    use anyhow::Context;
    //   use hypersync_client::simple_types::Log;

    //  use hypersync_client::{Client, Config, FieldSelection, TransactionSelection};
    use alloy_json_abi::Function;
    use hypersync_client::CallDecoder;
    use hypersync_client::QueryResponse;
    use rain_out::cli::Networks;

    use hypersync_format::Hex;
    use rain_out::api::{OutputResults, summerize_result};
    use rain_out::rpc::ChainClient;
    //  use alloy_dyn_abi::ext::abi::JsonAbiExt;
    //    use alloy_dyn_abi::JsonAbiExt;
    use rain_out::error::Error;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn test_ethereum() -> Result<(), Error> {
        println!("[Ethereum]creating client");
        let client = ChainClient::ethereum_client()?;
        let ress = client.latest_block().await;
        println!("[Ethereum]Latest block: {:?}", ress);
        assert!(
            ress > 21974719u64,
            "Latest block needs to be higher than 21974719"
        );
        println!("[Ethereum]Successfully got latest ethereum block");
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn testo() -> Result<(), Error> {
        println!("start");
        let address = "0xCEe8Cd002F151A536394E564b84076c41bBBcD4d";
        let block_stop = 38381480 + 500;
        let block_start = 38381480;
        let client = ChainClient::flare_client()?;
        let query = client.query_chain(block_start, block_stop, address).await?;

        let mut local: Vec<Log> = Vec::new();
        let mut resulto: Vec<OutputResults> = Vec::new();

        for entry in &query {
            // Iterate through each log batch in the page
            for log_batch in &entry.data.logs {
                // Iterate through individual logs in the batch
                for log in log_batch {
                    local.push(log.clone());
                    let block_nr = log.block_number.unwrap().encode_hex();
                    let br = block_nr.trim_start_matches("0x");
                    //   println!("Processed log: {:?}", );
                    match u64::from_str_radix(br, 16) {
                        Ok(num) => {
                            println!("Decoded value: {}", num);
                            resulto.push(OutputResults {
                                blocknr: num,
                                field_type: log
                                    .topics
                                    .first()
                                    .unwrap()
                                    .clone()
                                    .unwrap()
                                    .encode_hex(),
                            });
                        } // 26
                        Err(e) => println!("Error: {}", e),
                    }
                    println!(
                        "Got topic: {:?}",
                        log.topics.first().unwrap().clone().unwrap().encode_hex()
                    )
                }
            }
        }

        println!(
            "
        tx amount: {}
        ",
            local.len()
        );

        println!("Finished");
        Ok(())
    }

    /*
        #[tokio::test(flavor = "multi_thread")]
        async fn test_decode_input_with_single_signature() {
            let function =
                alloy_json_abi::Function::parse("transfer(address dst, uint256 wad)").unwrap();
            let function2 =
                alloy_json_abi::Function::parse("arb3(address,(uint256,uint256,uint256,((address,(address,address,bytes),(address,uint8,uint256)[],(address,uint8,uint256)[],bytes32),uint256,uint256,(address,uint256[],bytes)[])[],bytes),((address,address,bytes),(address,uint256[],bytes)[]))").unwrap();
            let input = "0xa9059cbb000000000000000000000000dc4bde73fa35b7478a574f78d5dfd57a0b2e22810000000000000000000000000000000000000000000000004710ca26d3eeae0a";
            let input = Data::decode_hex(input).unwrap();

            let input2 = "0x7ea0b76a000000000000000000000000cee8cd002f151a536394e564b84076c41bbbcd4d000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000009e00000000000000000000000000000000000000000000000000000000000000001ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000008c00000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000007c00000000000000000000000004d753bf7e0cc0353cf238b41231924c4fe1ea5f500000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000064000000000000000000000000000000000000000000000000000000000000006c06294add049626dd488b727fc04b819156502d588310c8b10ac36a9179b7c49490000000000000000000000005fb33d710f8b58de4c9fdec703b5c2487a5219d600000000000000000000000084c6e7f5a1e5dd89594cc25bef4722a1b8871ae6000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000005030000000000000000000000000000000000000000000000000000000000000012000000000000000000000000000000000000000000000003cb71f51fc55800000000000000000000000000000000000000000000000000000c7d713b49da0000914d696e20747261646520616d6f756e742e00000000000000000000000000008b616d6f756e742d7573656400000000000000000000000000000000000000000000000000000000000000000000000000000000000002c0bb3dd30c4e2000000000000000000000000000000000000000000000000080124610f0445a0000000000000000000000000000000000000000000000000000c328093e61ee400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b1a2bc2ec50000000000000000000000000000000000000000000000000004563918244f4000000000000000000000000000000000000000000000000000000e043da6172500008f6c6173742d74726164652d74696d65000000000000000000000000000000008d6c6173742d74726164652d696f0000000000000000000000000000000000008c696e697469616c2d74696d650000000000000000000000000000000000000000000000000000000000000000000000000000000000000006f05b59d3b200000000000000000000000000000000000000000000000000008ac7230489e80000000000000000000000000000000000000000000000000000006379da05b6000000000000000000000000000000000000000000000000000000354a6ba7a1800000000000000000000000000000000000000000000000000000000000000002830b00000024007400e0015801b401e001f40218025c0264080500040b20000200100001001000000b120003001000010b110004001000030b0100051305000201100001011000003d120000011000020010000003100404211200001d02000001100003031000010c1200004911000003100404001000012b12000001100003031000010c1200004a0200001a0b00090b1000060b20000700100000001000011b1200001a10000047120000001000001a1000004712000001100004011000002e12000001100006011000052e120000001000053d12000001100005001000042e1200000010000601100006001000032e120000481200011d0b020a0010000001100004011000072713000001100003031000010c12000049110000001000030010000247120000001000010b110008001000050110000800100001201200001f12000001100000011000094712000000100006001000073d120000011000002b12000000100008001000043b120000160901080b1000070b1000090110000a001000013d1200001b12000001100007001000013d1200000b10000a001000033a120000001000040010000248120001001000000b110008001000053d12000000100006001000042b1200000a0401011a1000000110000b031000010c1200004a020000001000000110000c031000010c1200004a020000040200010110000d031000010c12000049110000080300020110000b031000010c120000491100000110000c031000010c12000049110000100c01030110000f001000002e1200000110000e3e120000001000010010000100100001001000010010000100100001001000010010000100100001001000013d1a00000101000101100010010100010110001100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000fbda5f676cb37624f28265a144a48b0d6e87d3b60000000000000000000000000000000000000000000000000000000000000006f6dfdc43858b01c29d9310a4cc4faa4392dc9ce95461edc965cbe7a7fca18ebe00000000000000000000000000000000000000000000000000000000000000010000000000000000000000001d80c49bbbcd1c0911346656b529df9e5c2f783d0000000000000000000000000000000000000000000000000000000000000012f6dfdc43858b01c29d9310a4cc4faa4392dc9ce95461edc965cbe7a7fca18ebe000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000042021d80c49bbbcd1c0911346656b529df9e5c2f783d01ffff013bc1ecbcd645e525508c570a0ff04480a5614a86015376ffa8fbe804f3d9292bc6b319b0e59ce42311000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000002200000000000000000000000005fb33d710f8b58de4c9fdec703b5c2487a5219d600000000000000000000000084c6e7f5a1e5dd89594cc25bef4722a1b8871ae60000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000000000014b00000000000000000000000000000000000000000000000000000000000000060000000000000000000000007177b9d00bb5dbcaaf069cc63190902763783b098e756e6b6e6f776e2073656e6465720000000000000000000000000000000000000000000000000000000000000000000000000000000003478afcb67ff7400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000068bc3c5bac8200956d696e696d756d2073656e646572206f757470757400000000000000000000000000000000000000000000000000000000000000000000000000000000004b010000110400010110000103100000011000001e1200001d02000003100101011000033d12000003100001011000023d1200002b120000011000050110000400100000211200001d0200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
            let input2 = Data::decode_hex(input2).unwrap();

            let expected = function.abi_decode_input(input.as_ref(), false).unwrap();
            println!("Expected: {:?}", expected);
            let expected2 = function2.abi_decode_input(input2.as_ref(), false).unwrap();

            println!("Expected2: {:?}", expected2);
            let decoder =
                CallDecoder::from_signatures(&["transfer(address dst, uint256 wad)"]).unwrap();
            let decoder2 =
                CallDecoder::from_signatures(&["arb3(address,(uint256,uint256,uint256,((address,(address,address,bytes),(address,uint8,uint256)[],(address,uint8,uint256)[],bytes32),uint256,uint256,(address,uint256[],bytes)[])[],bytes),((address,address,bytes),(address,uint256[],bytes)[]))"]).unwrap();
            let got2 = decoder2.decode_input(&input2).unwrap().unwrap();
            println!("got2:{:?}", got2);

            let got = decoder.decode_input(&input).unwrap().unwrap();
            println!("got:{:?}", got);
            for (expected, got) in expected.iter().zip(got.iter()) {
                assert_eq!(expected, got, "Checking that decodes are the same");
            }
        }
    */
    #[tokio::test(flavor = "multi_thread")]
    async fn test_flare() -> Result<(), Error> {
        println!("[flare]creating client");
        let client = ChainClient::flare_client()?;
        let ress = client.latest_block().await;
        println!("[flare]Latest block: {:?}", ress);
        let _limit = 3;
        assert!(
            ress > 38284562u64,
            "[flare]latest block should be higher than 38284562"
        );
        println!("[flare]Successfully got latest flare block");
        let contract = rain_out::chains::FLARE_CONTRACT;
        let block_start: u64 = 38373380;
        let block_stop: u64 = 38379132; //38373381+1000;
        println!(
            "[flare]Checking query
input: 
        contract: {contract}
        Start block: {block_start}
        Stop block: {block_stop}
  "
        );
        const BALANCE_OF_SIGNATURE: &str =
            "function arb3(address orderBook, tuple takeOrders, tuple task)";
        let _signature = Function::parse(BALANCE_OF_SIGNATURE.as_ref())
            .context("parse function signature")
            .unwrap()
            .selector();
        println!("running decode");
        let _decoder = CallDecoder::from_signatures(&[BALANCE_OF_SIGNATURE]).unwrap();
        /*
        let query: Vec<Transaction> = client.query_chain(block_start, block_stop, contract).await?;
        println!("[flare]Amount of total transactions: {} ", query.len());

        for txn in &query {
            println!("decoder...");
            let try_dec = decoder.decode_input(&txn.clone().input.unwrap());
            println!("Raw TX: {:?}", try_dec);
            //  println!("Raw tx: {:?}", txn);
        }

        println!("displayed all tx");

        let successfulltx: Vec<Transaction> = query
            .iter()
            .filter(|tx| tx.status == Some(TransactionStatus::Success))
            .cloned() // Convert &Transaction to Transaction
            .collect();
        let failedtx: Vec<Transaction> = query
            .iter()
            .filter(|tx| tx.status == Some(TransactionStatus::Failure))
            .cloned() // Convert &Transaction to Transaction
            .collect();
        println!(
            "[flare]Amount of successfull transactions: {:?}
[flare]Amount of failed transactions: {:?}
[flare] Block limit: {:?}

",
            successfulltx.len(),
            failedtx.len(),
            limit,
        );
        println!("[flare]tx gathered ok");

        */
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_txn_query() -> Result<(), Error> {
        println!("testing txn summary");
        let client = ChainClient::flare_client()?;
        let res: Vec<QueryResponse> = client
            .query_chain(38373380, 38379132, rain_out::chains::FLARE_CONTRACT)
            .await?;

        // Calculate total capacity upfront
        let total_txs = res
            .iter()
            .flat_map(|res| &res.data.transactions)
            .map(|batch| batch.len())
            .sum();

        // Pre-allocate and flatten using consuming iterators
        let mut loot = Vec::with_capacity(total_txs);
        for response in res {
            loot.extend(response.data.transactions.into_iter().flatten());
        }

        let summary = summerize_result(loot, Networks::Flare);
        println!(
            "Summary is: 
Total tx: {}
failed_tx: {}
successful: {}
network: {}
    ",
            summary.total_tx, summary.failed_tx, summary.successful, summary.network
        );
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")] // hypersync library spawns threads sometimes and needs to support a multithreaded runtime
    async fn test_polygon() -> Result<(), Error> {
        println!("[polygon]creating client");
        let client = ChainClient::polygon_client()?;

        let ress = client.latest_block().await;
        println!("[polygon]Latest block: {:?}", ress);
        assert!(
            ress > 68678112u64,
            "[polygon]latest block should be higher than 68678112"
        );
        println!("[polygon]Successfully got latest polygon block");

        let block_start: u64 = 68678102;
        let block_stop: u64 = 68678122;
        let poly_contract = rain_out::chains::POLYGON_CONTRACT;
        println!(
            "[polygon]Building query test query with:
[polygon]Block start:      {block_start}
[polygon]Block stop:       {block_stop}
[polygon]Contract Address: {poly_contract}
"
        );
        Ok(())
        // let query = client.query_chain(block_start, block_stop, rainuptimetrack::chains::POLYGON_CONTRACT);
    }
}
