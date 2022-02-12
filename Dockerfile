FROM rust:1.58-alpine


COPY . .

RUN cargo build --release