# rainuptimetrack
track on-chain uptime

## Install Rust(if you do not have rust installed locally):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Install CapNproto:
https://capnproto.org/install.html  


## Install(Natively):   
```bash
git clone https://github.com/flipchan/rainuptimetrack
cd rainuptimetrack/ && cargo build --release
```


## Install with nix:  
todo


## Run:  
```bash
./target/release/rainuptimetrack 
error: the following required arguments were not provided:
  --network <NETWORK>
  --orderbook-address <ORDERBOOK_ADDRESS>
  --downtime-threshold <DOWNTIME_THRESHOLD>

Usage: rainuptimetrack --network <NETWORK> --orderbook-address <ORDERBOOK_ADDRESS> --downtime-threshold <DOWNTIME_THRESHOLD>

For more information, try '--help'.
```


See flare example

## Tests:  
```bash
make test
cargo test --release -- --nocapture 
warning: `/root/.cargo/config` is deprecated in favor of `config.toml`
note: if you need to support cargo 1.38 or earlier, you can symlink `config` to `config.toml`
   Compiling rainuptimetrack v0.1.0 (/tmp/rain/rainuptimetrack)
    Finished `release` profile [optimized] target(s) in 3.77s
     Running unittests src/lib.rs (target/release/deps/rainuptimetrack-609773e0416ab272)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/release/deps/rainuptimetrack-0b7ce4d38b28c81a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/client.rs (target/release/deps/client-600c91ed84e8ad5b)

running 4 tests
test tests::it_works ... ok
[Ethereum]creating client
[flare]creating client
[polygon]creating client
[Ethereum]Latest block: 21983052
[Ethereum]Successfully got latest ethereum block
test tests::test_ethereum ... ok
[polygon]Latest block: 68689284
[polygon]Successfully got latest polygon block
[polygon]Building query test query with:
[polygon]Block start:      68678102
[polygon]Block stop:       68678122
[polygon]Contract Address: 0x4aa9aef59c7b63cd5c4b2ede81f65a4225a99d9d

test tests::test_polygon ... ok
[flare]Latest block: 38336811
[flare]Successfully got latest flare block
[flare]Checking query
input: 
        contract: 0x5376Ffa8fbE804f3D9292bc6b319b0e59Ce42311
        Start block: 38277959
        Stop block: 38278392
  
scanned up to block 38278392
[flare]Amount of total transactions: 9 
[flare]Amount of successfull transactions: 7
[flare]Amount of failed transactions: 2
[flare]tx gathered ok
test tests::test_flare ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.15s

   Doc-tests rainuptimetrack

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Examples:   


#### Flare:  
```bash 
./target/release/rainuptimetrack -n flare -o 0xCEe8Cd002F151A536394E564b84076c41bBBcD4d -d 500 -b 38381480 38382480 
Network: Flare
Contract Address: 0xCEe8Cd002F151A536394E564b84076c41bBBcD4d
Block Range: 38381480 to 38382480
Downtime Theshold: 500 blocks
scanned up to block 38385699
[Threshold detected]Large gap between txs in 38382374 and 38383557: 1183 blocks
[Threshold detected]Large gap between txs in 38384377 and 38385214: 837 blocks
Total tx:       14
__finished__
```

#### Polygon:  
todo

#### Ethereum mainnet:  
todo

### FOrmat code:  
```bash
make fmt
```


### External References:  
-   https://docs.rainlang.xyz/intro
-   https://docs.envio.dev/docs/HyperIndex/hypersync
-   https://github.com/enviodev/hypersync-client-rust/   


