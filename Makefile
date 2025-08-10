make: debug
debug: build-debug test-debug
release: build-release test-release

build: build-debug

build-debug:
	cargo build --locked --workspace --all-targets
	cargo doc --locked --workspace --document-private-items

build-release:
	cargo build --locked --release --workspace --all-targets
	cargo doc --locked --release --workspace --document-private-items

test: test-debug

test-debug:
	cargo test --locked --workspace --all-targets --no-fail-fast

test-release:
	cargo test --locked --release --workspace --all-targets --no-fail-fast

fmt: format

format:
	cargo +nightly fmt --all
