#!/bin/sh

#set -ex
#cd "$(dirname $0)"

cargo +nightly build --target wasm32-unknown-unknown
#cargo +nightly run \
#  --bin wasm-bindgen -- \
#  target/wasm32-unknown-unknown/debug/wurst.wasm --out-dir .
wasm-bindgen target/wasm32-unknown-unknown/debug/wurst.wasm --out-dir .

npm install
npm run serve
