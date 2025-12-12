FROM debian:bookworm-slim

WORKDIR /app

COPY target/release/admin .

EXPOSE 3000

CMD ["admin"]