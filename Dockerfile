FROM rust:alpine3.16

WORKDIR /usr/chittychat

COPY . .

RUN cargo install --path .

CMD ["cargo", "run"]