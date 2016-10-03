install-toolchain:
	curl https://sh.rustup.rs -sSf | sh
	rustup target add x86_64-unknown-linux-musl

clean-build: clean build

clean:
	cargo clean

build:
	cargo build

build-release:
	cargo build --release

nightly-build:
	rustup run nightly cargo build --no-default-features --features nightly --verbose

build-static:
	cargo build --target=x86_64-unknown-linux-musl --release

test:
	cargo test

test-ignored:
	cargo test -- --ignored

doc:
	cargo doc --release -p ws -p jsonrpc-core -p libunicorn --no-deps

install:
	cargo install --force

.PHONY: clean build build-release build-static clean-build test test-ignored doc install install-toolchain
