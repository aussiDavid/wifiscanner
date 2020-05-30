FROM rust

RUN rustup component add rustfmt clippy --toolchain 1.43.1-x86_64-unknown-linux-gnu

USER 1000:1000

WORKDIR /usr/src/app

COPY . .

RUN cargo install cargo-watch

CMD ["make test-watch"]
