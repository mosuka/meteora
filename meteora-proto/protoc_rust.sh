#!/usr/bin/env bash

mkdir -p ./src/proto
rm -rf ./src/proto/*

GRPC_RUST_PLUGIN=$(which grpc_rust_plugin)

PROTO_DIRS=$(find . -name '*.proto' -print0 | xargs -0 -n1 dirname | sort | uniq)
for PROTO_DIR in ${PROTO_DIRS}
do
  PROTO_FILES=$(find "${PROTO_DIR}" -name '*.proto' -print0 | xargs -0 -n1 | sort | uniq | grep -v "${PROTO_DIR}/eraftpb.proto")
  protoc --proto_path="${PROTO_DIR}" --rust_out=./src/proto --grpc_out=./src/proto --plugin=protoc-gen-grpc=${GRPC_RUST_PLUGIN} ${PROTO_FILES}
done

RS_FILES=$(find ./src/proto -name '*.rs' -print0 | xargs -0 -n1 basename | sort | uniq)
echo "// This file is generated. Do not edit" > ./src/proto/mod.rs
echo "" >> ./src/proto/mod.rs

echo "use raft::eraftpb;" >> ./src/proto/mod.rs

echo "" >> ./src/proto/mod.rs
for RS_FILE in ${RS_FILES}
do
  MODULE_NAME=$(echo "${RS_FILE}" | awk -F'.' '{print $1}')
  echo "pub mod ${MODULE_NAME};" >> ./src/proto/mod.rs
done

cargo fmt
