FROM rust:1.83 AS builder
WORKDIR /usr/src
RUN rustup target add x86_64-unknown-linux-gnu

RUN USER=root cargo new fpa-server
WORKDIR /usr/src/fpa-server
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN cargo install --target x86_64-unknown-linux-gnu --path .

FROM debian:stable-slim
RUN apt update \
    && apt install -y openssl ca-certificates \
    && apt clean \
    && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*
WORKDIR /usr/share/fpa
COPY --from=builder /usr/local/cargo/bin/fpa-server .
COPY config.yaml config.yaml
COPY log4rs.yaml log4rs.yaml
RUN chmod +x /usr/share/fpa/fpa-server
USER 1000
EXPOSE 5000
CMD ["./fpa-server"]