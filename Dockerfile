FROM rust:latest AS builder

WORKDIR /app

ADD Cargo.toml .
ADD Cargo.lock .

RUN mkdir src benches && \
    echo 'fn main(){}' >  src/main.rs && \
    cargo fetch

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-gnu

FROM gcr.io/distroless/cc

COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/blackhole /bin/blackhole

ENTRYPOINT ["/bin/blackhole"]
