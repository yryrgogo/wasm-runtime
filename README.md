# wasm-interpreter

[WIP] WebAssembly Interpreter を Rust で書く。

## Todo

- [x] add 関数の wasm モジュールのバイナリを評価する
- [x] フィボナッチ数列を出力する関数の wasm モジュールのバイナリを評価して、任意の項の値を返す
- [ ] スマートポインタを使って Number や Value 構造体をもっと使いやすい形にリファクタする
- [ ] 次に実装するオペコードを決める
- [ ] モジュールの切り方含めリファクタする
- [ ] evaluator のテストをもう少し書く
- [ ] evaluator, decoder 以外でテストを書くべきモジュールを決める
- [ ] Rust の所有権、借用、ライフタイムが適切と思える実装に修正する

## Reference

### WebAssembly Official

- [WebAssembly Specification](https://webassembly.github.io/spec/core/index.html)
- [WebAssembly Design](https://github.com/WebAssembly/design)

### Other

- [Go 言語でつくるインタプリタ](https://www.oreilly.co.jp/books/9784873118222/)
- [Writing A Compiler In Go](https://compilerbook.com/)
