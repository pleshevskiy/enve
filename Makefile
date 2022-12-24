all: test build

test:
	cargo test --all-features

watch-test:
	cargo watch -x 'test --all-features'

build: fmt clippy
	cargo build

fmt:
	cargo fmt -- --check

clippy:
	cargo clippy

help:
	cat Makefile
