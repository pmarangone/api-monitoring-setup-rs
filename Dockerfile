# Stage 1: Build Stage
FROM rust:1.75-alpine AS builder

# Set the working directory
WORKDIR /app

# Install dependencies
RUN apk update && apk add --no-cache \
    pkgconf \
    libpq-dev \
    gcc \
    musl-dev \
    make

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs  # Dummy file to cache dependencies
RUN cargo build --release && rm -r src 

# Copy the actual source code and rebuild
COPY src ./src
RUN cargo build --release

# Stage 2: Runtime Stage
FROM alpine:latest

# Install necessary runtime dependencies
RUN apk update && apk add --no-cache libpq-dev

# Set the working directory
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/rust-app .

# Expose the application port (adjust if needed)
EXPOSE 8000

# Run the application
CMD ["./rust-app"]
