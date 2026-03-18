# ===================== Builder stage ======================
FROM rust:latest AS builder

WORKDIR /build

# Install sqlx-cli for running migrations
RUN cargo install sqlx-cli --no-default-features --features sqlite

# Cache dependencies by building a dummy project first
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./backend-derive ./backend-derive
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Create var directory and .env file for the build process
RUN mkdir -p var/db
RUN echo "DATABASE_URL=sqlite:/build/var/db/turnkeeper.db" > .env && \
    echo "GM_JWT_SECRET=dev-gm-secret-key-change-in-production" >> .env && \
    echo "USER_JWT_SECRET=dev-user-secret-key-change-in-production" >> .env && \
    echo "GM_PASSWORD=dev-password" >> .env

# Create the SQLite database and run migrations
COPY ./migrations ./migrations
RUN sqlx database create
RUN sqlx migrate run

# Now copy the actual source code
COPY ./src ./src

# Build the application in release mode (with optimizations)
RUN cargo build --release

# ====================== Runtime stage ======================
FROM debian:bookworm-slim

WORKDIR /app

# Create var directory
RUN mkdir -p var/db

# Copy the compiled binary from builder
COPY --from=builder /build/target/release/turnkeeper_backend ./

# Copy the initialized database from builder
COPY --from=builder /build/var/db/turnkeeper.db var/db/

# Copy the .env.local file
COPY .env.local .env

# Set environment variables for the application
ENV DATABASE_URL=sqlite:var/db/turnkeeper.db

# Expose the port the application listens on
EXPOSE 8080

# Run the application with exec to ensure logs are captured
CMD ["sh", "-c", "exec ./turnkeeper_backend"]
