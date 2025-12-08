# Builder stage
FROM rust:1-bookworm as builder

WORKDIR /usr/src/app
COPY . .

# Build the application in release mode
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install OpenSSL/CA certificates required for reqwest and curl for health checks
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/calendar-filter /usr/local/bin/calendar-filter

ENV PORT=3000


CMD ["calendar-filter"]
