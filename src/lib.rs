pub mod scanner;
pub mod parser;
pub mod ream;

pub use parser::*;
pub use scanner::*;
pub use ream::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn ream2ast(source: &str) -> String {
    let mut parser = Parser::new(&source);
    let result = parser.parse_entry();
    let output = serde_json::to_string(&result).unwrap();
    // let output_raw = serde_json::to_string(&output).unwrap();

    output
}

#[wasm_bindgen]
pub fn ream2csv(source: &str) -> String {
    let mut parser = Parser::new(&source);
    let result = parser.parse_entry();
    let output_raw = match result {
        Ok(entry) => {
            entry.unwrap().to_csv()
        },
        err => serde_json::to_string(&err).unwrap(),
    };

    output_raw
}
