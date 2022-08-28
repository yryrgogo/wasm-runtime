#!/bin/bash

set -eu -o pipefail

cargo run ./src/wasm/module.wasm
cargo run ./src/wasm/const.wasm

echo "OK"
