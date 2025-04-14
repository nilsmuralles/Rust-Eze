# Build stage
FROM rust:latest as builder

WORKDIR /app

# Accept the build argument
ARG DATABASE_URL

ENV DATABASE_URL=$DATABASE_URL

COPY . .

RUN cargo build --release

# Production stage
FROM debian:bookworm-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/rust-eze .

CMD ["./rust-eze"]
