# Builder stage
FROM rust:1-alpine AS builder

WORKDIR /usr/src/app

# Install build dependencies
RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig

COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build statically linked binary
ENV RUSTFLAGS="-C target-feature=+crt-static"
RUN cargo build --release --target x86_64-unknown-linux-musl

# Runtime stage
FROM alpine:latest

# Install minimal runtime dependencies
RUN apk add --no-cache ca-certificates curl

# Copy the statically linked binary
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/calendar-filter /usr/local/bin/calendar-filter

ENV PORT=3000

EXPOSE 3000

CMD ["calendar-filter"]
