FROM debian:bookworm-slim

WORKDIR /app

COPY target/release/admin /app/admin

RUN chmod +x /app/admin

EXPOSE 3000

ENTRYPOINT ["/app/admin"]
