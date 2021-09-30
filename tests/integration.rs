use std::fs;
use ream::parser::{Parser};

#[test]
fn test_1() {
    let source = fs::read_to_string("./tests/test_1.ream").unwrap();
    let mut parser = Parser::new(&source);
    let result = parser.parse_entry();
    let output = result.expect("a").expect("b").to_csv_str().expect("c");

    println!("{:?}", &output);
    let answer = fs::read_to_string("./tests/test_1.csv").unwrap();
    println!("{:?}", &answer);
    assert_eq!(output, answer);
}

