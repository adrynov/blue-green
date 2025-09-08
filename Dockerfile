# Build stage
FROM rust:1.89.0-slim AS build

# Install MUSL support
RUN apt-get update && apt-get install -y musl-tools pkg-config libssl-dev && \
  rustup target add x86_64-unknown-linux-musl

WORKDIR /app

# Copy source and build
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

# Final stage
FROM scratch
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/blue-green /

EXPOSE 8080
ENTRYPOINT ["/blue-green"]

ARG COLOUR
ENV COLOUR=$COLOUR

