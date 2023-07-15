# Setting
#===============================================================
SHELL := /bin/bash
OS    := $(shell uname | tr A-Z a-z)

# Option
#===============================================================
CARGO_OPTIONS           :=
CARGO_SUB_OPTIONS       :=
APP_ARGS                := --port 8080 --address 0.0.0.0

# Environment
#===============================================================
export RUST_BACKTRACE=1

# Task
#===============================================================
deps: ## Install depend tools
	rustup component add rustfmt
	rustup component add clippy
	rustup component add rust-src
	cargo $(CARGO_OPTIONS) install --force cargo-outdated
	cargo $(CARGO_OPTIONS) install --force cargo-audit
	rustup show

run: fix fmt clippy ## Execute a main.rs
	cargo $(CARGO_OPTIONS) run $(CARGO_SUB_OPTIONS) -- $(APP_ARGS)

test: fix fmt clippy ## Run the tests
	cargo $(CARGO_OPTIONS) test $(CARGO_SUB_OPTIONS) -- --nocapture

check: fix fmt ## Check syntax, but don't build object files
	cargo $(CARGO_OPTIONS) check $(CARGO_SUB_OPTIONS)

build: ## Build all project
	cargo $(CARGO_OPTIONS) build $(CARGO_SUB_OPTIONS)

release-build: ## Build all project
	cargo $(CARGO_OPTIONS) build --release $(CARGO_SUB_OPTIONS)

check-lib: ## Check module version
	cargo $(CARGO_OPTIONS) outdated -R

update: ## Update modules
	cargo $(CARGO_OPTIONS) update

clean: ## Remove the target directory
	cargo $(CARGO_OPTIONS) clean

fix: ## Run fmt
	cargo $(CARGO_OPTIONS) fix --allow-staged --allow-dirty $(CARGO_SUB_OPTIONS)

fmt: ## Run fmt
	cargo $(CARGO_OPTIONS) fmt

fmt-check: ## Run fmt
	cargo $(CARGO_OPTIONS) fmt --all -- --check

clippy: ## Run clippy
	cargo $(CARGO_OPTIONS) clippy --all-features $(CARGO_SUB_OPTIONS) -- -D warnings

bench: ## Run benchmark
	cargo $(CARGO_OPTIONS) bench

audit: ## Audit your dependencies for crates with security vulnerabilities reported
	cargo $(CARGO_OPTIONS) audit

publish:
ifeq ($(LEVEL),)
	$(error LEVEL not set correctly.)
endif
	cargo release $(LEVEL) --no-dev-version --tag-name "{{version}}"

help: ## Print help
	echo -e "Usage: make [task]\n\nTasks:"
	perl -nle 'printf("    \033[33m%s%-20s\033[0m %s\n",$$1,$$2,$$3) if /^([a-zA-Z]){1}([a-zA-Z_-]*?):(?:.+?## )?(.*?)$$/' $(MAKEFILE_LIST)

# Config
#===============================================================
.SILENT: help
# If you want `Target` instead of `Task`, you can avoid it by using dot(.) and slash(/)
# ex) node_modules: => ./node_modules:
.PHONY: $(shell egrep -o '^(_)?[a-zA-Z-]+:' $(MAKEFILE_LIST) | sed 's/://')
.DEFAULT_GOAL := build
