FROM rustlang/rust:stable as builder

WORKDIR /app

RUN USER=root cargo new rubble
WORKDIR /app/rubble

COPY Cargo.toml Cargo.lock ./

RUN echo 'fn main() { println!("Dummy") }' > ./src/main.rs

RUN cargo build --release

RUN rm -r target/release/.fingerprint/rubble-*

COPY src src/
COPY migrations migrations/
COPY templates templates/

RUN cargo build --release --frozen --bin rubble

FROM rustlang/rust:stable

RUN mkdir /application


COPY --from=builder /app/rubble/migrations /application/migrations
COPY --from=builder /app/rubble/templates /application/templates
COPY --from=builder /app/rubble/target/release/rubble /application/rubble

EXPOSE 8000

ENV DATABASE_URL postgres://root@postgres/application

WORKDIR /application
CMD ["./rubble"]