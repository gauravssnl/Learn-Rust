#! /bin/bash

WABT_BIN=$HOME/wabt/bin
BINARYEN=$HOME/binaryen/bin
TARGET=wasm32-unknown-unknown
NAME=do_addition
BUILDMODE=release
BINARY=target/$TARGET/$BUILDMODE/$NAME.wasm

cargo build --target $TARGET --$BUILDMODE
$WABT_BIN/wasm-strip $BINARY
mkdir -p www
$BINARYEN/wasm-opt -o www/$NAME.wasm -Oz $BINARY