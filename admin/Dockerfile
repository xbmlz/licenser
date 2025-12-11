FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/admin/target/release/admin /app/admin

EXPOSE 3000

CMD ["/app/admin"]