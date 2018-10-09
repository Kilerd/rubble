FROM clux/muslrust:nightly as builder

COPY . /app

WORKDIR /app

RUN cargo build --release


FROM clux/diesel-cli

COPY --from=builder /app/migrations /application/migrations
COPY --from=builder /app/Rocket.toml /application/Rocket.toml
COPY --from=builder /app/entrypoint.sh /application/entrypoint.sh
COPY --from=builder /app/static /application/static
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rubble /application/rubble

EXPOSE 8000

ENV ROCKET_ENV production
ENV DATABASE_URL postgres://root@postgres/rubble

WORKDIR /application
CMD ["sh", "./entrypoint.sh"]