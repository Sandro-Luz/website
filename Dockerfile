# Use a Rust image as a base
FROM rust:1.75.0-slim

# Set the working directory
WORKDIR /app

# Copy the entire project
COPY . .

# Build the project
RUN cargo build --release


CMD ["./target/release/website_sg"]
EXPOSE 5555