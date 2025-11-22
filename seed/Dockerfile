FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /usr/src

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
RUN apt-get update && apt-get -y upgrade && apt-get install -y libssl-dev openssl ca-certificates
COPY --from=planner /usr/src/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin seed --bin migration

FROM debian:sid
RUN apt-get update && apt-get -y upgrade && apt-get install -y libssl-dev openssl ca-certificates
COPY --from=builder /usr/src/target/release/seed /usr/local/bin/seed
COPY --from=builder /usr/src/target/release/migration /usr/local/bin/migration
WORKDIR /usr/local/bin
CMD ["sh", "-c", "migration up --verbose && seed"]
