FROM rust:latest

WORKDIR /app/

COPY . .

RUN rustup default

RUN cargo install diesel-cli --no-default-features --features postgres
RUN cargo install watch

CMD [ "cargo", "watch", "--why", "--", "echo" ]