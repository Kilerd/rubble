FROM rust:1.29

RUN cargo install diesel_cli --no-default-features --features postgres

EXPOSE 8000

COPY . /app

WORKDIR /app

RUN cargo build --release

ENTRYPOINT ["sh", "./entrypoint.sh"]