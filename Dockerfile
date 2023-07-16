FROM rust:latest AS builder

WORKDIR /app

ADD Cargo.toml .
ADD Cargo.lock .

RUN mkdir src benches && \
    echo 'fn main(){}' >  src/main.rs && \
    cargo fetch

COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc

COPY --from=builder /app/target/release/blackhole /bin/blackhole

ENTRYPOINT ["/bin/blackhole"]
