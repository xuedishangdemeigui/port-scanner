#!/bin/sh -e

BINARY_NAME='port-scanner'

RUSTFLAGS='-C target-feature=+crt-static' cargo build -r

cp ./target/release/${BINARY_NAME} ./

