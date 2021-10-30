pub mod error;
pub mod parser;
pub mod format;
pub mod scanner;
pub mod decorator;

pub use parser::*;
pub use scanner::*;
// pub use ream::*;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn ream2ast(source: &str) -> Result<JsValue, JsValue> {
    if source == "" {
        return Ok(JsValue::NULL);
    };

    let mut parser = Parser::new(&source);

    let ast: Result<JsValue, JsValue> = match parser.parse_entry() {
        Err(e) => Err(JsValue::from_serde(&e).unwrap()),
        Ok(opt_entry) => match opt_entry {
            None => Ok(JsValue::NULL),
            Some(entry) => Ok(JsValue::from_serde(&entry).unwrap()),
        }
    };

   ast 
}

#[wasm_bindgen]
pub fn ream2csv(source: &str) -> JsValue {
    if source == "" {
        return JsValue::NULL;
    };

    let mut parser = Parser::new(&source);

    let result = match parser.parse_entry() {
        Err(e) => JsValue::from_serde(&e).unwrap(),
        Ok(opt_entry) => match opt_entry {
            None => JsValue::NULL,
            Some(entry) => {
                let list = entry.to_csv_list().unwrap(); // TODO: unwrap?
                JsValue::from_serde(&list).unwrap()
            }
        }
    };

    result
}
