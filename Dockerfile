FROM rust:1.67 as builder
WORKDIR /usr/src/chittychat_back
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y apt-transport-https  && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/chittychat_back /usr/local/bin/chittychat_back
CMD ["chittychat_back"]