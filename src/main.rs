mod scanner;

use std::env;
use scanner::*;
use std::fs;

fn main() {

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("can't read file");
    let mut scanner = Scanner::new(contents.chars());

    let tokens = scanner.scan();

    println!("{:?}", tokens);





}
