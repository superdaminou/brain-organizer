FROM rust:1.81

RUN apt-get update 
RUN apt-get -y install gcc mono-mcs clang llvm libclang-dev
RUN apt-get -y install cmake g++ make 

RUN cargo install wasm-pack && cargo install --locked trunk
RUN rustup target add wasm32-unknown-unknown
WORKDIR /brain_app
COPY . .
RUN cargo build

CMD ["cargo", "run","--", "web"]