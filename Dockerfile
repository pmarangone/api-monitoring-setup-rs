# Stage 1: Build Stage
FROM rust:1.75 AS builder

# Set the working directory
WORKDIR /app

# Install system dependencies required for building
RUN apt-get update && apt-get install -y pkg-config libpq-dev && rm -rf /var/lib/apt/lists/*

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs  # Dummy file to cache dependencies
RUN cargo build --release && rm -r src 

# Copy the actual source code and rebuild
COPY src ./src
RUN cargo build --release

# Stage 2: Runtime Stage
FROM debian:bullseye-slim

# Install only necessary runtime dependencies
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/rust-app .

# Expose the application port (adjust if needed)
EXPOSE 8000

# Run the application
CMD ["./rust-app"]
