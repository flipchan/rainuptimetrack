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
	./target/release/rainuptimetrack -n flare -o 0x5376Ffa8fbE804f3D9292bc6b319b0e59Ce42311 -d 2 -b 38277959 38278392