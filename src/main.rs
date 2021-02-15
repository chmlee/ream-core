mod scanner;
mod parser;
mod ream;

use scanner::*;
use parser::*;
use std::fs;

fn main() {

    let file = fs::read_to_string("./example/test.md").unwrap();

    let mut parser = Parser::new(&file);

    let result = parser.parse_entry();



    println!("{:?}", result);


}
