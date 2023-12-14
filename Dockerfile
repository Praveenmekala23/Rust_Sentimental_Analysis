# Use the official Rust image from Docker Hub
FROM rust:1.74.1-slim

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Rust project files to the container
COPY . .

# Install necessary dependencies
RUN cargo build

# Start the program using cargo run
CMD ["cargo", "run"]
