mod scanner;
mod parser;

use std::env;
use std::fs;
use scanner::*;
use parser::*;

fn main() {

    // let args: Vec<String> = env::args().collect();
    // let filename = &args[1];
    // let contents = fs::read_to_string(filename)
    //     .expect("can't read file");
    // let mut scanner = Scanner::new(contents.chars());

    // let tokens = scanner.scan();

    // println!("{:?}", tokens);

    let content = r#"# Country
- name: "USA"
## State
- name: "New York"
### City
- name: "New York City"

## State
- name: "Illinois"

### City
- name: "Chicago"
"#;

    let parser = Scanner::new(content.chars());

    // println!("{}", content);
    // println!("{:?}", &parser);



}
