FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD . /blisp
WORKDIR /blisp/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /blisp/fuzz/target/x86_64-unknown-linux-gnu/release/blisp-fuzz /