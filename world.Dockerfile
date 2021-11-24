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
	cargo build --release --bin world && \
	cp target/release/world /world

FROM debian:bullseye-slim

COPY --from=builder /world /

EXPOSE 50051

ENTRYPOINT ["/world"]
