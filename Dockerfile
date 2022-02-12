FROM rust:1.58-alpine as app


COPY . .

RUN cargo build --release && ls -la target/release

CMD ./target/release/rust-helm-test


