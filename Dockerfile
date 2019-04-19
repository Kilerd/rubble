FROM clux/muslrust:stable as builder

COPY . /app
WORKDIR /app

RUN cargo build --release

FROM alpine:latest

COPY --from=builder /app/migrations /application/migrations
COPY --from=builder /app/templates /application/templates
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rubble /application/rubble

EXPOSE 8000

ENV DATABASE_URL postgres://root@postgres/rubble

WORKDIR /application
CMD ["./rubble"]