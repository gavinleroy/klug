pub mod expr;
pub mod stmt;
pub mod decl;
pub mod literal;

use expr::Expr;
use stmt::Stmt;
use logos::Logos;
use std::iter::Peekable;
use crate::lexer::{Lexer, SyntaxKind};
use crate::syntax::KlugLanguage;

#[derive(Debug)]
pub struct ParseError {
    msg: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MSG: {}", self.msg)
    }    
}

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    has_error: bool,
}

impl<'a> Parser<'a> {
    pub fn new(s: &'a str) -> Self {
        Self { 
            lexer: Lexer::new(s).peekable(), 
            has_error: false,
        }
    }

    pub fn parse(mut self) -> Parse {
        let mut decls = Vec::<Decl>::new();

        while !self.is_end() {
            decls.push(Decl::new(&mut self));
        }

        Parse { declarations: decls }
    }

    fn peek(&mut self) -> Option<SyntaxKind> {
        self.lexer.peek().map(|(kind, _)| *kind)
    }

    fn next(&mut self) -> (SyntaxKind, &str) {
        self.lexer.next().unwrap()
    }

    fn consume(&mut self) {
        let _ = self.lexer.next();
    }

    fn expect(&mut self, sk: SyntaxKind) -> Result<String, ParseError> {
        if self.peek() != Some(sk) {
            return Err(ParseError { 
                msg: format!("Expected {} but got {}", sk, self.next().map(|(_, txt)| txt)) 
            })
        } else {
            let (_, txt) = self.next();
            Ok(txt)
        }
    }

    fn synchronize(&mut self) {
        // TODO consume input until we are  on a decl boundary
        self.has_error = true;
        loop {
            match self.peek() {
                None => break,
                _ => p.consume(),
            }
        }
    }

    fn is_end(&mut self) -> bool {
        self.peek() == None
    }

    fn error_occurred<T>(&mut self, obj: &T) 
        where T: HasError {
            // TODO this doesn't feel right
            obj.synchronize(&mut *self);
    }
}

#[derive(Debug, PartialEq)]
pub struct Parse {
    pub(crate) declarations: Vec<Decl>,
}

#[cfg(test)]
mod tests {
    use super::*;
    // TODO tests maybe?
}
