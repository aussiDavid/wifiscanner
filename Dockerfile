FROM rust

USER 1000:1000

WORKDIR /usr/src/app

COPY . .

RUN cargo install cargo-watch

CMD ["make test-watch"]