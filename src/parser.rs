use crate::scanner::*;
use crate::ream::*;

#[derive(Debug)]
pub struct Parser<'source> {
    pub scanner: Scanner<'source>,
}

impl<'source> Parser<'source> {
    pub fn new(source: &'source str) -> Self {
        Parser {
            scanner: Scanner::new(source),
        }
    }

    pub fn parse_entry(&mut self) -> ParseEntryResult {

        let level = match self.scanner.take_token()? {
            Some(Token(TokenType::Header(n))) => n,
            Some(_) => panic!("expecting header level"),
            None => panic!("xxx"),
        };

        let class = match self.scanner.take_token()? {
            Some(Token(TokenType::Class(c))) => c,
            _ => panic!("expecting class name"),
        };

        let mut entry = Entry::new(class, level);

        // loop for variables
        while let Some(Token(TokenType::Dash)) = self.scanner.take_token()? {
            let result = self.parse_variable()?;
            match result {
                Some(var) => entry.push_variable(var),
                None => panic!("expecting variables"),
            }
        }


        Ok(Some(entry))
    }

    pub fn parse_variable(&mut self) -> ParseVariableResult {

        let key = match self.scanner.take_token()? {
            Some(Token(TokenType::Key(k))) => k,
            _ => panic!("expecting key"),
        };

        self.parse_symbol(TokenType::Colon)?;

        let value = match self.scanner.take_token()? {
            Some(Token(TokenType::String(v))) => v,
            _ => panic!("expecting value"),
        };



        Ok(Some(Variable::new(key, Value::String(value))))
    }

    pub fn parse_symbol(&mut self, tt: TokenType) -> Result<(),()> {
        match self.scanner.take_token()? {
            Some(tt) => Ok(()),
            _ => panic!("expecting symbol {:?}", tt),
        }
    }










}
