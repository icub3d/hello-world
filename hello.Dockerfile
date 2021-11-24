# syntax=docker/dockerfile:1.3

FROM rust:slim AS builder

RUN rustup component add rustfmt

WORKDIR /usr/src/hello-world

COPY Cargo.toml Cargo.lock .
RUN mkdir .cargo src && \
	echo "// dummy file" >src/lib.rs && \
	cargo vendor >.cargo/config.toml

COPY . .

RUN --mount=type=cache,target=/usr/src/hello-world/target \
	cargo build --release --bin hello && \
	cp target/release/hello /hello

FROM debian:bullseye-slim

COPY --from=builder /hello /

EXPOSE 50051

ENTRYPOINT ["/hello"]
