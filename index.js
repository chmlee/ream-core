import wasm from "./Cargo.toml";

async function loadWasm() {
  const exports = await wasm();
  console.log(exports.ream2ast);
}
