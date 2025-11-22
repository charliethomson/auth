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
RUN cargo build --release --bin rest

FROM debian:sid

RUN apt-get update && apt-get -y upgrade && apt-get install -y libssl-dev openssl ca-certificates curl
COPY --from=builder /usr/src/target/release/rest /usr/local/bin/rest
ENV PORT=8080
EXPOSE 8080

CMD ["rest"]
