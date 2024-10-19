FROM rust:1.81 as builder


RUN apt-get update && apt-get install -y musl-tools gcc-aarch64-linux-gnu

WORKDIR /app

# Copy the Cargo files and cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

# Add musl targets for Intel and ARM architectures
RUN rustup target add x86_64-unknown-linux-musl 
RUN apt update && apt install -y musl-tools musl-dev
RUN apt-get install -y build-essential
RUN yes | apt install gcc-x86-64-linux-gnu
# Copy the rest of the source code
COPY . .

# Build the project for both Intel and ARM using musl for static linking
ARG TARGET_ARCH=x86_64-unknown-linux-musl
ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
RUN cargo build --release --target $TARGET_ARCH

FROM debian:buster-slim
RUN apt-get update

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/swapi-rs /usr/local/bin/swapi-rs

RUN useradd -m appuser

# Copy the statically linked binary from the builder

USER appuser

EXPOSE 8000

ENTRYPOINT ["swapi-rs"]





