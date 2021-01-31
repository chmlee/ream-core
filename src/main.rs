mod scanner;

use scanner::*;

fn main() {

    let text = String::from(r#"
#      Title
- key   : "value"
        "#);
    let mut scanner = Scanner::new(text.chars());

    scanner.scan();

    println!("{:?}", scanner.tokens);





}
