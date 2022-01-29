FROM rust:1.58.1-slim-bullseye as builder
WORKDIR /usr/src/wabper

COPY ./packages ./packages
COPY ./server ./server
COPY ./docker/entrypoint.sh ./

RUN apt-get update && apt-get install libpq-dev -y
RUN cargo install --path server
RUN strip -s /usr/local/cargo/bin/wabper-server

FROM debian:buster-slim
RUN apt update && apt install libpq-dev -y && apt clean
WORKDIR /usr/bin
COPY --from=builder /usr/local/cargo/bin/wabper-server ./wabper-server
COPY --from=builder /usr/src/wabper/entrypoint.sh /

ENTRYPOINT [ "/entrypoint.sh" ]
