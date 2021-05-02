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
    let output = result.expect("a").expect("b").to_ast_str_pretty().expect("c");

    output
}

#[wasm_bindgen]
pub fn ream2csv(source: &str) -> String {
    let mut parser = Parser::new(&source);
    let result = parser.parse_entry();
    let output = result.expect("a").expect("b").to_csv_str().expect("c");

    output
}
