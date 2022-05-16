const fs = require("fs");
const path = require("path");

const modulePath = path.resolve(__dirname, "ifElseSimple.wasm");
const bytes = fs.readFileSync(modulePath);
const value_1 = parseInt(process.argv[2]);

const run = async () => {
  const obj = await WebAssembly.instantiate(new Uint8Array(bytes), { console });
  obj.instance.exports.ifElseSimple(value_1);
};

run();
