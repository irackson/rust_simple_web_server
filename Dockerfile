# Use the official Rust image as the base image
FROM rust:latest

# Create a new directory for the app
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the project
RUN cargo build --release

# Run the application
CMD ["./target/release/myrustwebapp"]

# Expose the port
EXPOSE 3000
