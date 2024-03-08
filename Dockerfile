# Stage 1: Build the Rust application
FROM rust:latest as builder

WORKDIR /usr/src/app

# Copy the entire source code
COPY . .

# Build the application
RUN cargo build --release

# Stage 2: Create the final minimal image
FROM debian:bookworm

WORKDIR /usr/src/app

# Copy only the necessary files from the build stage
COPY --from=builder /usr/src/app/target/release/lookbusy .

# Set environment variables if needed
# ENV EXAMPLE_ENV_VAR=value

# Run the application
CMD ["./lookbusy"]
