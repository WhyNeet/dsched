FROM rust:1.94.1-alpine AS chef
RUN cargo install cargo-chef
WORKDIR /app

# Analyze full workspace
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Cache dependencies
FROM chef AS cacher
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Build only the target binary
FROM chef AS builder
COPY . .
COPY --from=cacher /app/target target

RUN cargo build --release -p master --bin master

FROM alpine:latest AS runtime

WORKDIR /app
COPY --from=builder /app/target/release/master .

ENTRYPOINT [ "./master" ]
