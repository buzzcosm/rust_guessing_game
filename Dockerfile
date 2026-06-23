FROM rust:alpine AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:latest
COPY --from=builder /app/target/release/guessing_game /usr/local/bin/guessing_game
CMD ["guessing_game"]
