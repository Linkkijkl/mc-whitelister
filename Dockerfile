FROM rust:alpine AS builder
RUN apk add --no-cache musl-dev
WORKDIR /app

# Backend
RUN rustup default nightly
ENV RUSTFLAGS="-C target-feature=-crt-static"
# Future debugger: add your additional Rust specific files to the bellow.
# Specifying files makes Docker use cache if they are not modified,
# skipping the tidiously long build.
COPY Cargo.lock Cargo.toml ./
COPY src src
COPY templates templates
RUN cargo build --release --bin mc-whitelister

# Final container
FROM alpine AS runtime
RUN apk add --no-cache libgcc
WORKDIR /app
COPY --from=builder /app/target/release/mc-whitelister .
USER 1000
CMD [ "./mc-whitelister" ]
