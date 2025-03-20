VERSION=1.2
NAME=rainup-makefile

check:
	cargo hack check --no-dev-deps --release

test:
	cargo test --release  --all-features  -- --nocapture 

build: 
	cargo build --release --all-features

fmt: 
	cargo +nightly fmt --all

fuzz:
	cd fuzz && cargo +nightly build && cargo +nightly fuzz run csv_fuzz

run_flare:
	build
	./target/release/rainuptimetrack -n flare -o 0xCEe8Cd002F151A536394E564b84076c41bBBcD4d -d 30 -b 38381480 38382480

testtxn:
	cargo test test_txn_query  --release --all-features -- --nocapture

clippy:
	cargo clippy --all-targets --all-features -- -D warnings
