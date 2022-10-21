FROM rust:1.63.0-slim

RUN apt-get update && \
    apt-get -y install libssl-dev pkg-config
