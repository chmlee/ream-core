pub mod error;
pub mod parser;
pub mod format;
pub mod scanner;
pub mod decorator;

pub use parser::*;
pub use scanner::*;
// pub use ream::*;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn ream2ast(source: &str) -> String {
    let mut parser = Parser::new(&source);
    let result = parser.parse_entry();
    let output = result
        .expect("a")
        .expect("b")
        .to_ast_str_pretty()
        .expect("c");

    output
}

#[wasm_bindgen]
pub fn ream2csv(source: &str) -> String {
    let mut parser = Parser::new(&source);
    let result = parser.parse_entry();
    let output = result.expect("a").expect("b").to_csv_str().expect("c");

    output
}
