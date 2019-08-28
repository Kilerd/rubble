FROM clux/muslrust:stable as builder

WORKDIR /app

RUN USER=root cargo new rubble
WORKDIR /app/rubble

COPY Cargo.toml Cargo.lock ./

RUN echo 'fn main() { println!("Dummy") }' > ./src/main.rs

RUN cargo build --release

RUN rm -r target/x86_64-unknown-linux-musl/release/.fingerprint/rubble-*

COPY src src/
COPY migrations migrations/
COPY templates templates/

RUN cargo build --release --frozen --bin rubble


FROM alpine:latest

COPY --from=builder /app/rubble/migrations /application/migrations
COPY --from=builder /app/rubble/templates /application/templates
COPY --from=builder /app/rubble/target/x86_64-unknown-linux-musl/release/rubble /application/rubble

EXPOSE 8000

ENV DATABASE_URL postgres://root@postgres/rubble

WORKDIR /application

CMD ["./rubble"]