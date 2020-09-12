FROM rust:latest

COPY Cargo.* ./
COPY src/ ./src
COPY benches/ ./benches 

RUN cargo build --release
RUN cat target/release/libbrcode.so > libbrcode.so
ENTRYPOINT [ "/bin/bash" ]