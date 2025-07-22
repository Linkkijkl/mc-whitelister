# https://github.com/devcontainers/images/tree/main/src/rust

# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="bullseye"
FROM rust:1-${VARIANT}

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive

# Install clippy and rustfmt
RUN rustup component add clippy-preview
RUN rustup component add rustfmt
