#!/bin/sh

# For more coments about what's going on here, see the `hello_world` example

#set -ex

cargo +nightly build --target wasm32-unknown-unknown &&
wasm-bindgen ../../target/wasm32-unknown-unknown/debug/dom.wasm --out-dir . &&
npm install &&
npm run serve
