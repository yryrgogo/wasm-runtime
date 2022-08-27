# wasm-runtime

implements the WebAssembly runtime with Rust.


## tool

check Wasm bytecode

`od -tx1 hoge.wasm`

Disassemble function body

`wasm-objdump -d hoge.wasm`

Print raw section contents

`wasm-objdump -s hoge.wasm`
