use crate::scanner::*;
use std::str::Chars;

pub struct Parser<'a, T>
where
    T: Iterator<Item = char>
{
    pub scanner: Scanner<T>,
}

impl<'a, T> Parser<'a, T>
where
    T: Iterator<Item = char>
{
    // pub fn new(content: &str) -> Self {
    //     Parser {
    //         scanner: Scanner::new(content.chars())
    //     }
    // }

    pub fn test<'a>(source: String) -> Scanner<'a, std::str::Chars> {
        Scanner::new(source.chars())
    }



}
