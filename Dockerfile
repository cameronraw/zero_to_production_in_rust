FROM rust:1.63.0

WORKDIR /app

RUN apt update && apt install lld clang -y

COPY . .

ENV SQLX_OFFLINE true

RUN cargo build --release

ENV APP_ENVIRONMENT production

ENTRYPOINT ["./target/release/zero_to_prod"]
