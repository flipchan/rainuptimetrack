VERSION=1.0
NAME=rainuptimetrack-makefile

check:
	cargo hack check --no-dev-deps --release

test:
	cargo test --release -- --nocapture 

build: 
	cargo build --release

fmt: 
	cargo +nightly fmt --all

run_flare:
	build
	./target/release/rainuptimetrack -n flare -o 0xCEe8Cd002F151A536394E564b84076c41bBBcD4d -d 30 -b 38381480 38382480



