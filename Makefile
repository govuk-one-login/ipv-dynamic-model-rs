.PHONY: check

check:
	cargo build
	cargo test
	cargo fmt --check
	dx fmt --check
	cargo clippy -- -D clippy::pedantic -D clippy::nursery
