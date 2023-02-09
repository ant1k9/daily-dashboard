.PHONY: lint
lint:
	@cargo clippy --all-targets --all-features -- -D warnings

.PHONY: test
test:
	@cargo test

install:
	@cargo install --path .
