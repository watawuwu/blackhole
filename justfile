default:
    just --list

# Install depend tools
deps:
    rustup component add rustfmt
    rustup component add clippy
    rustup component add rust-src
    cargo install --force cargo-outdated
    cargo install --force cargo-audit
    rustup show

# Execute a main.rs
run arg="--port 8080 --address 0.0.0.0": fix fmt clippy
    cargo run -- {{ arg }}

# Run the tests
test: fix fmt clippy
    cargo test -- --nocapture

# Check syntax, but don't build object files
check: fix fmt
    cargo check

# Build all project
build:
    cargo build

# Build all project
release-build:
    cargo build --release

# Check module version
check-lib:
    cargo outdated -R

# Update modules
update:
    cargo update

# Remove the target directory
clean:
    cargo clean

# Run fmt
fix:
    cargo fix --allow-staged --allow-dirty

# Run fmt
fmt:
    cargo fmt

# Run fmt
fmt-check:
    cargo fmt --all -- --check

# Run clippy
clippy:
    cargo clippy --all-features -- -D warnings

# Run benchmark
bench:
    cargo bench

# Audit your dependencies for crates with security vulnerabilities reported
audit:
    cargo audit

# Build container
container version=0.6.2:
    echo docker buildx build --platform=linux/amd64,linux/arm64 -t ghcr.io/watawuwu/blackhole:{{version}} --push .
