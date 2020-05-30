.PHONY: check format test test-watch lint

check:
	@cargo check

format:
	@cargo fmt

test:
	@cargo test

test-watch:
	@cargo watch -x test

lint:
	@cargo clippy

checks: check format test lint
	@git status
	@echo looks good enough to raise a PR 👍
	@echo awesome work! 😍
