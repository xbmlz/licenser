FROM rust:1.90-bullseye as builder
WORKDIR /src
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /src/target/release/admin .

EXPOSE 3000

ENTRYPOINT ["./admin"]
