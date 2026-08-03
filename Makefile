.PHONY: check

check:
	cargo build
	cargo test
	cargo fmt --check
	cargo clippy -- -D clippy::pedantic -D clippy::nursery
