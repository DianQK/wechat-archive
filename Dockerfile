FROM rust:latest as builder

RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown

WORKDIR /usr/src/wechat-archive

COPY . .

RUN cargo build --release --manifest-path ./server/Cargo.toml && cp ./server/target/release/wechat-archive .
RUN trunk --config ./frontend/Trunk.toml build --release --public-url /static/

FROM debian:stable-slim as runner
RUN apt-get update && apt-get install -y sqlcipher && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/wechat-archive/wechat-archive /usr/local/bin/wechat-archive
COPY --from=builder /usr/src/wechat-archive/dist /dist

VOLUME /config
VOLUME /assets

ENV MYSQL_HOSTNAME "db"

CMD ["wechat-archive", "--config","/config/wechat-archive.toml", "--port", "8080", "--static-dir", "/dist", "--assets-dir", "/assets"]
EXPOSE 8080
