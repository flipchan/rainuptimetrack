#![no_main]
use libfuzzer_sys::fuzz_target;
// fuzz the csv output
use csv::Writer;
use rain_out::fuzz_types::ChainResultFuzz;
use std::path::Path;
use std::fs::File;



fuzz_target!(|summary: ChainResultFuzz| {
    // fuzzed code goes here
    let output_path = Path::new(".");
    let filen = File::create(output_path.join("fuzz_test.csv")).unwrap();
    let mut blocks_writer = Writer::from_writer(filen);
    blocks_writer.serialize(summary).unwrap();


});
