.PHONY: fmt lint test check demo

fmt:
	cargo fmt

lint:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	cargo test --all-targets --all-features

check: fmt lint test

demo:
	bash examples/demo.sh
