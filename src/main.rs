mod scanner;
mod parser;

use std::env;
use std::fs;
use scanner::*;
use parser::*;

fn main() {

    let file = fs::read_to_string("./example/test.md").unwrap();

    let mut scanner = Scanner::new(&file);

    scanner.get_source();
    scanner.scan_line().unwrap();
    scanner.get_source();
    scanner.scan_line().unwrap();
    println!("{:?}", scanner);


}
