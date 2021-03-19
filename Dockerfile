ARG RUST_VERSION=1.50.0

FROM rust:${RUST_VERSION}-slim-buster AS builder

WORKDIR /repo

RUN set -ex \
    && apt-get update \
    && apt-get install -y --no-install-recommends \
       build-essential \
       clang \
       cmake \
       git \
       # For rocksdb
       libgflags-dev \
       libsnappy-dev \
       zlib1g-dev \
       libbz2-dev \
       liblz4-dev \
       libzstd-dev \
       # For protobuf
       protobuf-compiler \
       golang-go \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# build rocksdb
RUN git clone https://github.com/facebook/rocksdb.git -b v6.11.4 \
    && cd rocksdb \
    && make static_lib \
    && make install

COPY . ./

RUN rustup component add rustfmt --toolchain ${RUST_VERSION}-x86_64-unknown-linux-gnu \
    && cargo install --version 2.22.1 protobuf-codegen \
    && cargo build --release

FROM debian:bullseye-slim

WORKDIR /

RUN set -ex \
    && apt-get update \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /repo/bin /usr/local/bin
COPY --from=builder /usr/local/lib/librocksdb.a /usr/local/lib/librocksdb.a

EXPOSE 5000 7000

ENTRYPOINT [ "meteora" ]
CMD [ "start" ]
