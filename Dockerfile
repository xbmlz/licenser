FROM debian:bookworm-slim

WORKDIR /app

COPY admin .

EXPOSE 3000

CMD ["admin"]