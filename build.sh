#!/bin/sh

#set -ex
#cd "$(dirname $0)"

#cargo +nightly run \
#  --bin wasm-bindgen -- \
#  target/wasm32-unknown-unknown/debug/wurst.wasm --out-dir .
# enure upto date binary: cargo install -f wasm-bindgen-cli --git https://github.com/rustwasm/wasm-bindgen.git
cargo +nightly build --target wasm32-unknown-unknown --verbose &&
wasm-bindgen target/wasm32-unknown-unknown/debug/wurst.wasm --out-dir . &&
npm install &&
npm run serve
