mod scanner;

use scanner::*;

fn main() {

    let text = String::from(r#"# Title
- list:
  * "item 1"
  * "item 2"
- key: "value"
- list:
  * "item 1"
  * "item 2"
"#);
    let mut scanner = Scanner::new(text.chars());

    let tokens = scanner.scan();

    println!("{}", text);
    println!("{:?}", tokens);





}
