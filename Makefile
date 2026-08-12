.PHONY: check

check:
	cargo fmt --check
	# dx fmt --check # broken, cargo fmt makes changes dx fmt --check rejects
	cargo build
	cargo clippy -- -D clippy::pedantic -D clippy::nursery
	cargo test
