# Use the official Rust image as the base image
FROM rust:latest

# Create a new directory for the app
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Build dependencies
RUN cargo build --release

# Copy the source code
COPY . .

# Build the project
RUN cargo install --path .

# Run the application
CMD ["your_project_name"]

# Expose the port
EXPOSE 3000
