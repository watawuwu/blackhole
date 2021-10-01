FROM rust:1.55.0 AS builder

WORKDIR /app

ADD Makefile .
ADD Cargo.toml .
ADD Cargo.lock .

RUN mkdir src benches && \
    echo 'fn main(){}' >  src/main.rs && \
    echo 'fn main(){}' >  benches/benchmark.rs && \
    cargo fetch

COPY . .

RUN make deps release-build CARGO_BUILD_TARGET=x86_64-unknown-linux-gnu

FROM gcr.io/distroless/cc

COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/blackhole /bin/blackhole

ENTRYPOINT ["/bin/blackhole"]
