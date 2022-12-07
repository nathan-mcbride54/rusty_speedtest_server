FROM rust:1.59.0

WORKDIR /usr/src/speedtest
COPY . .

RUN cargo install --path .

EXPOSE 443

CMD ["speedtest-server"]
