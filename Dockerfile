# Frontend builder stage
FROM rust:1-slim as frontend-builder

# Install build dependencies for frontend
RUN apt-get update && apt-get install -y pkg-config libssl-dev curl && rm -rf /var/lib/apt/lists/*

# Install trunk for building the frontend
RUN cargo install trunk

# Install wasm target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

# Copy frontend files
COPY frontend_visit_counter/ ./frontend_visit_counter/
COPY static/ ./static/

# Build the frontend (from within the frontend directory)
WORKDIR /app/frontend_visit_counter
RUN trunk build --release

# Backend builder stage
FROM rust:1-slim as backend-builder

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# First copy the local prisma_auth dependency
COPY prisma_auth/ ./prisma_auth/

# Copy backend files
COPY backend_visit_counter/ ./backend_visit_counter/
COPY static/ ./static/
COPY assets/ ./assets/

# Build the backend (from within the backend directory, not workspace)
WORKDIR /app/backend_visit_counter
RUN cargo build --release

# Final runtime stage
FROM debian:bookworm-slim

# Create necessary directories
RUN mkdir -p /data

# Copy the compiled binary from the backend builder stage
COPY --from=backend-builder /app/backend_visit_counter/target/release/backend_visit_counter /usr/local/bin/visit_counter

# Copy the built frontend from the frontend builder stage
COPY --from=frontend-builder /app/frontend_visit_counter/dist/ /app/frontend/

# Copy static assets
COPY static/ /static/

# Expose the port that Rocket will use (default is 8000)
EXPOSE 8000

# Run the application
CMD ["visit_counter"]
