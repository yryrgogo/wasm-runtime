const fs = require("fs");
const path = require("path");

const modulePath = path.resolve(__dirname, "addFloat.wasm");
const bytes = fs.readFileSync(modulePath);
const value_1 = parseFloat(process.argv[2]);
const value_2 = parseFloat(process.argv[3]);

const run = async () => {
  const obj = await WebAssembly.instantiate(new Uint8Array(bytes));
  let add_value = obj.instance.exports.addFloat(value_1, value_2);
  console.log(`${value_1} + ${value_2} = ${add_value}`);
  console.log(value_1 + value_2);
};

run();
