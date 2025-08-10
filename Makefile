make: debug
debug: build-debug test-debug
release: build-release test-release

build: build-debug

build-debug:
	cargo build --all-targets
	cargo doc --document-private-items

build-release:
	cargo build --release --all-targets
	cargo doc --release --document-private-items

test: test-debug

test-debug:
	cargo test --locked --all-targets --no-fail-fast

test-release:
	cargo test --locked --release --all-targets --no-fail-fast

fmt: format

format:
	cargo +nightly fmt --all
