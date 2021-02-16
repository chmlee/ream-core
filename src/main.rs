mod scanner;
mod parser;
mod ream;

use scanner::*;
use parser::*;
use std::{env, fs};

fn main() {

    let args: Vec<String> = env::args().collect();

    let path = if args.len() == 2 {
        &args[1]
    } else {
        "./example/test.md"
    };

    let file = fs::read_to_string(path).unwrap();

    let mut parser = Parser::new(&file);

    let result = parser.parse_entry().unwrap().ok_or_else(|| panic!("a"));

    // println!("{:#?}", &file);
    println!("{:#?}", &result);

    // let se = serde_json::to_string_pretty(&result).unwrap();
    // println!("{}", &se);

}
