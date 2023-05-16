FROM --platform=linux/amd64 rustlang/rust:nightly as builder

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update 
RUN apt-get install -y cmake clang
RUN cargo install cargo-fuzz

ADD . /delaunator-rs/

WORKDIR /delaunator-rs/fuzz/

RUN cargo +nightly fuzz build

FROM --platform=linux/amd64 rustlang/rust:nightly

COPY --from=builder /delaunator-rs/fuzz/target/x86_64-unknown-linux-gnu/release/point /

COPY --from=builder /delaunator-rs/fuzz/target/x86_64-unknown-linux-gnu/release/triangulate /