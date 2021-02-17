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
            Some(Token(TokenType::Header(n), _, _)) => n,
            _ => panic!("expecting header level"),
        };

        Ok(level)
    }

    pub fn parse_token_identifier(&mut self) -> Result<String, ()> {
        let identifier = match self.scanner.take_token()? {
            Some(Token(TokenType::Class(c), _, _))
            | Some(Token(TokenType::Key(c), _, _))   => c,
            _ => panic!("expecting identifier"),
        };

        Ok(identifier)
    }

    pub fn parse_entry(&mut self) -> ParseEntryResult {
        println!("parsing new entry");

        let level = self.parse_token_header()?;
        self.level = level;

        let class = self.parse_token_identifier()?;

        let mut entry = Entry::new(class, level);

        // loop for variables
        while let Some(Token(TokenType::Dash, _, _)) = self.scanner.peek_token()? {
            self.scanner.take_token()?;
            // println!("{:?}", self.scanner.buffer);
            let result = self.parse_variable()?;
            match result {
                Some(var) => entry.push_variable(var),
                None => panic!("expecting variables"),
            }
        }

        // println!("{:?}", self.scanner.buffer);

        // loop for subentries
        let mut subentries: Vec<Entry> = vec![];
        while let Some(Token(TokenType::Header(next_level), _, _)) = self.scanner.peek_token()? {
            if *next_level == self.level + 1 {               // child
                let subentry = match self.parse_entry()? {
                    Some(sub) => sub,
                    None => panic!("expecting subentry"),
                };
                subentries.push(subentry);
            } else if *next_level <= self.level {
                self.level -= 1;
                break;
            } else {
                panic!("wrong level of entry");
            }
        }
        entry.subentries = subentries;

        Ok(Some(entry))
    }

    pub fn parse_variable(&mut self) -> ParseVariableResult {

        let key = self.parse_token_identifier()?;

        self.parse_symbol_colon()?;

        let value = match self.scanner.take_token()? {
            Some(Token(TokenType::String(v), _, _)) => v,
            _ => panic!("expecting value"),
        };

        Ok(Some(Variable::new(key, Value::String(value))))
    }

    pub fn parse_symbol_colon(&mut self) -> Result<(),()> {
        match self.scanner.take_token()? {
            Some(Token(TokenType::Colon, _, _)) => Ok(()),
            _ => panic!("expecting Colon"),
        }
    }


}
