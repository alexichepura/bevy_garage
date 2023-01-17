# rustup target add aarch64-unknown-linux-gnu
# cargo build --release --target aarch64-unknown-linux-gnu
FROM debian:buster-slim
COPY ./target/release/api /usr/local/bin/api
CMD ["api"]
