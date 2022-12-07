FROM rust:1.59.0

WORKDIR /usr/src/mws-speedtest
COPY . .

RUN cargo install --path .

EXPOSE 8080

CMD ["mws-speedtest-server"]
