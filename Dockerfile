FROM rust:1.81

RUN apt-get update && \
    apt-get -y install gcc mono-mcs clang llvm libclang-dev sql

RUN cargo install wasm-pack
RUN cargo install --locked trunk

RUN rustup target add wasm32-unknown-unknown

RUN cargo init brain_app

COPY ./Cargo.toml /brain_app/Cargo.toml

WORKDIR /brain_app
RUN cargo build

CMD ["cargo", "run","--", "web"]