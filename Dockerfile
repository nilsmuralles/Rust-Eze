FROM rust:latest

# Set the working directory inside the rust-eze project
WORKDIR /app/rust-eze

# Copy Cargo files and build dependencies separately for better caching
COPY rust-eze/Cargo.toml rust-eze/Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -r src

# Copy the entire project
COPY rust-eze ./

# Build the Rust application
RUN cargo build --release

# Set the correct path for the compiled binary
CMD ["./target/release/rust-eze"]

