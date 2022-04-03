# wasm-interpreter

[WIP] WebAssembly Interpreter を Rust で書く。

## Todo

- [ ] フィボナッチ数列を出力する関数の wasm モジュールのバイナリを評価して、任意の項の値を返す
- [ ] 次に実装するオペコードを決める
- [ ] Rust の所有権、借用、ライフタイムが適切と思える実装に修正する
- [ ] String を多用しているが、参照にできる箇所が多くあるはず
- [ ] Value の実装が微妙なので単一の struct でうまく抽象化したい

## Reference

### WebAssembly Specification

https://webassembly.github.io/spec/core/index.html

### WebAssembly Design

https://github.com/WebAssembly/design
