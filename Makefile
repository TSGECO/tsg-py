.PHONY: clean build test lint pre-commit sanitize all help

# Set help as the default target
.DEFAULT_GOAL := help

# add build target to the default target
all: build  ## Build everything (default)

clean:  ## Remove build artifacts
	rm -rf build dist

test: build  ## Run all tests (Python and Rust)
	cargo test --all-targets --all-features
	pytest -vls tests

build: clean  ## Build the project
	uv sync
	uv tool run maturin develop -r
	cargo run --bin stub_gen
	mv tsgraph.pyi tsgraph
	ruff check --fix --unsafe-fixes

lint:  ## Run linters
	cargo clippy
	ruff check --fix --unsafe-fixes

pre-commit:  ## Run pre-commit hooks
	pre-commit run --all-files

sanitize: lint pre-commit  ## Run all code quality checks

help:  ## Display this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'
