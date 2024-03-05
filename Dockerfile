# Use an official Rust image as a parent image
FROM rust:1.68 as builder

# Create a working directory
WORKDIR /usr/src/server

# Copy the current directory contents into the container
COPY . .

# Build your application
RUN cargo install --path .

# Use the official Debian slim image for a lean production container
FROM debian:buster-slim

# Copy the binary from the builder stage to the final image
COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server

# Set the PORT environment variable
ENV PORT=8080

# When the container starts, run the binary
CMD ["server"]

