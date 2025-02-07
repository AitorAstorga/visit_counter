# Use an official Rust image as the builder.
FROM rust:1-slim as builder

# Create a new empty shell project and copy our files.
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/
COPY assets/ ./assets/

# Build the application in release mode.
RUN cargo build --release

# Use a minimal base image for the final binary.
FROM debian:bookworm-slim

# Copy the compiled binary from the builder stage.
COPY --from=builder /app/target/release/visit_counter /usr/local/bin/

# Expose the port that Rocket will use (default is 8000).
EXPOSE 8000

# Run the application.
CMD ["visit_counter"]
