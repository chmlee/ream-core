use crate::scanner::*;
use crate::ream::*;

#[derive(Debug)]
pub struct Parser<'source> {
    pub scanner: Scanner<'source>,
    pub level: usize,
}

impl<'source> Parser<'source> {
    pub fn new(source: &'source str) -> Self {
        Parser {
            scanner: Scanner::new(source),
            level: 0,
        }
    }

    pub fn parse_token_header(&mut self) -> Result<usize, ()> {
        let level = match self.scanner.take_token()? {
            Some(Token(TokenType::Header(n))) => n,
            _ => panic!("expecting header level"),
        };

        Ok(level)
    }

    pub fn parse_token_identifier(&mut self) -> Result<String, ()> {
        let identifier = match self.scanner.take_token()? {
            Some(Token(TokenType::Class(c))) => c,
            Some(Token(TokenType::Key(c)))   => c,
            _ => panic!("expecting identifier"),
        };

        Ok(identifier)
    }

    pub fn parse_entry(&mut self) -> ParseEntryResult {

        let level = self.parse_token_header()?;
        self.level = level;

        let class = self.parse_token_identifier()?;

        let mut entry = Entry::new(class, level);

        // loop for variables
        while let Some(Token(TokenType::Dash)) = self.scanner.take_token()? {
            let result = self.parse_variable()?;
            match result {
                Some(var) => entry.push_variable(var),
                None => panic!("expecting variables"),
            }
        }

        // loop for subentries
        if let Some(Token(TokenType::Header(next_level))) = self.scanner.take_token()? {
            println!("{}", next_level);
        }



        Ok(Some(entry))
    }

    pub fn parse_variable(&mut self) -> ParseVariableResult {

        let key = self.parse_token_identifier()?;

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
