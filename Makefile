# works on macOS
default:
	cargo build --release --locked

run:
	cargo run

lint:
	cargo clippy --tests -- -D warnings

format-check:
	cargo fmt --all -- --check

check:
	cargo check --locked

format:
	cargo fmt --all

test:
	cargo test --locked