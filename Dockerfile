FROM rust:latest

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -r src

COPY . ./
RUN cargo build --release

CMD ["./target/release/my_rust_app"]
