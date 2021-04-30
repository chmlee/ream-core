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

#[wasm_bindgen]
pub fn ream2csv(source: &str) -> String {
    let mut parser = Parser::new(&source);
    let result = parser.parse_entry().unwrap().ok_or_else(|| panic!("something went wrong"));
    result.unwrap().to_csv()
}

pub fn add_one(x: f64) -> f64 {
    x + 1.0
}

pub fn ream2csv_test(source: &str) -> String {
    let mut parser = Parser::new(&source);
    let result = parser.parse_entry().unwrap().ok_or_else(|| panic!("something went wrong"));
    result.unwrap().to_csv()
}
