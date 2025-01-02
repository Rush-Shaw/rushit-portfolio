# Step 1: Use the Rust 1.83 image to build the application
FROM rust:1.83 AS builder

# Step 2: Set the working directory
WORKDIR /app

# Step 3: Copy the Cargo manifest and fetch dependencies
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

# Step 4: Copy the entire project and build in release mode
COPY . .
RUN cargo build --release

# Step 5: Use a newer Debian runtime image with updated glibc
FROM debian:bookworm-slim

# Step 6: Install dependencies required by Rocket
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl && \
    rm -rf /var/lib/apt/lists/*

# Step 7: Set the working directory and copy files
WORKDIR /app
COPY --from=builder /app/target/release/rushit-portfolio .
COPY --from=builder /app/static ./static

# Step 8: Set proper permissions
RUN chmod +x ./rushit-portfolio && \
    chown -R nobody:nogroup /app

# Step 9: Set environment variables
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

# Step 10: Switch to non-root user
USER nobody

# Step 11: Expose port
EXPOSE 8000

# Step 12: Run the application
CMD ["./rushit-portfolio"]