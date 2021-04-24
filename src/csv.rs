// pub mod scanner;
// pub mod ream;

// use scanner::*;
use crate::parser::*;
use crate::ream::*;
// use ream::*;

// use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
pub fn ream2csv(source: &str) -> Result<Entry, ()> {
    let mut parser = Parser::new(&source);
    let result = parser.parse_entry().unwrap().ok_or_else(|| panic!("something went wrong"));
    result
}
