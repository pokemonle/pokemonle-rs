FROM rust:1-slim AS builder

# Install dependencies
RUN apt-get update && \
    apt-get install -y libssl-dev pkg-config libclang-dev clang libsqlite3-dev libpq-dev && \
    rm -rf /var/lib/apt/lists/*

ADD . /app
WORKDIR /app

# Build the application
RUN cargo build --release --workspace

FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y ca-certificates sqlite3 postgresql && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/pokemonle* /usr/local/bin/
ENTRYPOINT [ "/usr/local/bin/pokemonle-http" ]