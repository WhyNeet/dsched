FROM rust:1.94.1-alpine AS chef
RUN cargo install cargo-chef
RUN cargo install sqlx-cli --no-default-features --features rustls,postgres --version 0.8.6

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

ENV SQLX_OFFLINE=true
RUN cargo build --release -p control-plane --bin control-plane

FROM alpine:latest AS runtime

WORKDIR /app
COPY --from=builder /app/target/release/control-plane .


ENV APP_HTTP_PORT=8080 APP_TCP_PORT=8081
EXPOSE 8080 8081
ENTRYPOINT [ "./control-plane" ]
