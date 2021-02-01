mod scanner;

use scanner::*;

fn main() {

    let text = String::from(r#" # Title
## bla "#);
    let mut scanner = Scanner::new(text.chars());

    let tokens = scanner.scan();

    println!("{:?}", text);
    println!("{:?}", tokens);





}
