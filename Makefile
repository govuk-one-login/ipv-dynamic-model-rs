.PHONY: check

check:
	cargo fmt --check
	# dx fmt --check # broken, `cargo fmt` and `dx fmt` conflict, see https://github.com/DioxusLabs/dioxus/issues/3433
	cargo build
	cargo clippy -- -D clippy::pedantic -D clippy::nursery
	cargo test

fix-check:
	cargo fmt
	# dx fmt --check # broken, `cargo fmt` and `dx fmt` conflict, see https://github.com/DioxusLabs/dioxus/issues/3433
	cargo build
	cargo clippy -- -D clippy::pedantic -D clippy::nursery
	cargo test

dependencies:
	@which rustup || (echo "You must install rust, visit https://rustup.rs" && false)
	@cargo binstall --help > /dev/null 2>&1 || (echo "You must install the \"binstall\" plugin for cargo, visit https://github.com/cargo-bins/cargo-binstall" && false)
	rustup update
	rustup toolchain install stable
	rustup target add wasm32-unknown-unknown
	cargo binstall dioxus-cli

serve:
	(cd ui-dioxus && dx serve)
