FROM ghcr.io/evanrichter/cargo-fuzz AS builder

ADD . /blisp
WORKDIR /blisp/fuzz
RUN cargo +nightly update -p unicode-segmentation --precise 1.12.0 && \
    cargo +nightly fuzz build

FROM debian:bookworm
COPY --from=builder /blisp/fuzz/target/x86_64-unknown-linux-gnu/release/blisp-fuzz /
