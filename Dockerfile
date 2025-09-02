# Multi-stage build for smaller image
FROM rust:1.89.0-alpine AS builder

# Install dependencies
RUN apk add --no-cache musl-dev

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src


# Build release binary
RUN cargo build --release --target x86_64-unknown-linux-musl

# Runtime image
FROM alpine:latest
RUN apk add --no-cache ca-certificates

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/json-validator .
COPY static ./static

# Create non-root user
RUN addgroup -g 1000 appuser && \
    adduser -D -s /bin/sh -u 1000 -G appuser appuser

USER appuser

EXPOSE 8080

CMD ["./json-validator"]
