# M1 https://github.com/messense/homebrew-macos-cross-toolchains
# rustup target add x86_64-unknown-linux-gnu
# cargo build --package=api --release --target x86_64-unknown-linux-gnu
FROM --platform=linux/amd64 debian:buster-slim
COPY ./target/x86_64-unknown-linux-gnu/release/api /usr/local/bin/api
EXPOSE 3000
CMD ["api"]
