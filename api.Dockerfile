# syntax=docker/dockerfile:1.3

FROM rust:slim AS builder

RUN rustup component add rustfmt

WORKDIR /usr/src/hello-world

COPY Cargo.toml Cargo.lock .
RUN mkdir .cargo src && \
	echo "// dummy file" >src/lib.rs && \
	cargo vendor >.cargo/config.toml

COPY . .

ENV ROCKET_ENV=prod

RUN --mount=type=cache,target=/usr/src/hello-world/target \
	cargo build --release --bin api && \
	cp target/release/api /api

FROM debian:bullseye-slim

COPY --from=builder /api /

ENV ROCKET_ENV=prod
ENV ROCKET_ADDRESS=0.0.0.0

EXPOSE 8000

ENTRYPOINT ["/api"]
