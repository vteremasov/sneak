build:
	echo "Start Building"
	cargo build -p sneak --release --verbose

test: clippy fmtCheck
	echo "Start testing"
	cargo test

run:
	echo "Running"
	cargo run

fmtCheck:
	cargo fmt -- --check

clippy:
	cargo clippy -- -D warnings

fmt:
	cargo fmt --all

