pub mod scanner;
pub mod parser;
pub mod ream;

use parser::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn ream2json(source: &str) -> String {
    let mut parser = Parser::new(&source);
    let result = parser.parse_entry().unwrap().ok_or_else(|| panic!("something went wrong"));
    let se = serde_json::to_string(&result).unwrap();
    se
}
