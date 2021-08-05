import rust from "@wasm-tool/rollup-plugin-rust";

export default {
    input: {
        foo: "Cargo.toml",
    },
    output: {
      dir: 'output',
      format: 'iife',
    },
    plugins: [
        rust(),
    ],
};
