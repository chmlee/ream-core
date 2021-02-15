mod scanner;
mod parser;
mod ream;

use scanner::*;
use parser::*;
use std::{env, fs};

fn main() {

    // let args: Vec<String> = env::args().collect();
    // let file = fs::read_to_string(&args[1]).unwrap();

    // let mut parser = Parser::new(&file);

    // let result = match parser.parse_entry().unwrap() {
    //     Some(r) => r,
    //     None => panic!("bla"),
    // };

    // println!("{:#?}", &result);

    // let se = serde_json::to_string_pretty(&result).unwrap();
    // println!("{}", &se);



    let mut parser = Parser::new(&"#");

    let result = match parser.parse_entry().unwrap() {
        Some(r) => r,
        None => panic!("bla"),
    };

    println!("{:#?}", &result);

    let se = serde_json::to_string_pretty(&result).unwrap();
    println!("{}", &se);

}
