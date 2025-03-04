# Stage 1: Build Stage
FROM rust:1.83-alpine AS builder

# Set the working directory
WORKDIR /app

# Install dependencies
RUN apk add --no-cache \
    pkgconf \
    postgresql-dev \
    gcc \
    musl-dev \
    make

# Cache dependencies first
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() { println!("Hell world!"); }' > src/main.rs
RUN cargo build --release || true  
RUN rm -r src  


COPY src ./src
RUN cargo clean && cargo build --release  


FROM alpine:latest


RUN apk add --no-cache postgresql-libs


WORKDIR /app


COPY --from=builder /app/target/release/rust-app .  

# Expose the application port
EXPOSE 8000

# Run the application
CMD ["./rust-app"]
