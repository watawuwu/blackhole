# Setting
#===============================================================
SHELL := /bin/bash
OS    := $(shell uname | tr A-Z a-z)

# Const
#===============================================================
name := blackhole

# Option
#===============================================================
SHELL                   := /bin/bash
LOG_LEVEL               := debug
PREFIX                  := $(HOME)/.cargo
LOG                     := $(shell echo '$(name)' | tr - _)=$(LOG_LEVEL)
TARGET                  := x86_64-apple-darwin
CARGO_BIN               := cross
ifneq (,$(findstring mingw64, $(OS)))
    CARGO_BIN := cargo
endif
ifeq (,$(shell command -v cross 2> /dev/null))
    CARGO_BIN := cargo
endif
CARGO_OPTIONS           :=
CARGO_SUB_OPTIONS       := --target $(TARGET)
CARGO_COMMAND           := $(CARGO_BIN) $(CARGO_OPTIONS)
CONTAINER_REPO          := watawuwu/blackhole
CONTAINER_TAG           := latest
APP_ARGS                :=

# Environment
#===============================================================
export RUST_LOG=$(LOG)
export RUST_BACKTRACE=1

# Task
#===============================================================
deps: ## Install depend tools
	rustup target add $(TARGET)
	rustup component add rustfmt
	rustup component add clippy
	rustup show # for container

dev-deps: ## Install dev depend tools
	rustup component add rust-src
	$(CARGO_COMMAND) install --force cargo-outdated

run: fix fmt clippy ## Execute a main.rs
	$(CARGO_COMMAND) run $(CARGO_SUB_OPTIONS) -- $(APP_ARGS)

test: fix fmt clippy ## Run the tests
	$(CARGO_COMMAND) test $(CARGO_SUB_OPTIONS) -- --nocapture

test4ci: fmt-check clippy ## Run the tests
	$(CARGO_COMMAND) test $(CARGO_SUB_OPTIONS)

check: fix fmt ## Check syntax, but don't build object files
	$(CARGO_COMMAND) check $(CARGO_SUB_OPTIONS)

build: fmt-check clippy ## Build all project
	$(CARGO_COMMAND) build $(CARGO_SUB_OPTIONS)

check-lib: ## Check module version
	$(CARGO_COMMAND) outdated

update: ## Update modules
	$(CARGO_COMMAND) update

clean: ## Remove the target directory
	$(CARGO_COMMAND) clean

install: ## Install to $(PREFIX) directory
	$(CARGO_COMMAND) install --force --root $(PREFIX) --path . $(CARGO_SUB_OPTIONS)

fix: ## Run fmt
	$(CARGO_COMMAND) fix $(CARGO_SUB_OPTIONS) --allow-dirty

fmt: ## Run fmt
	$(CARGO_COMMAND) fmt

fmt-check: ## Run fmt
	$(CARGO_COMMAND) fmt --all -- --check

clippy: ## Run clippy
	$(CARGO_COMMAND) clippy --all-features -- -D warnings

release-build: ## Build all project
	$(MAKE) build CARGO_SUB_OPTIONS="$(CARGO_SUB_OPTIONS) --release"

publish:
ifeq ($(LEVEL),)
	$(error LEVEL not set correctly.)
endif
	cargo release $(LEVEL) --no-dev-version --tag-name "\{\{version\}\}"

container-build:
	docker build -t $(CONTAINER_REPO):$(CONTAINER_TAG) .

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
