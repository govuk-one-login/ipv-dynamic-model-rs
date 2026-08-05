.PHONY: check

check:
	cargo build
	cargo test
	cargo fmt --check
	# dx fmt --check # broken, cargo fmt makes changes dx fmt --check rejects
	cargo clippy -- -D clippy::pedantic -D clippy::nursery
