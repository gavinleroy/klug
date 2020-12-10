pub mod expr;
mod stmt;

use expr::Expr;
use stmt::Stmt;
use logos::Logos;
use std::iter::Peekable;
use crate::lexer::{Lexer, SyntaxKind};
use crate::syntax::KlugLanguage;

// This trait allows the parser to finish 
// consuming tokens until it is ccertain 
// parsing past this error has happened
trait HasError {
    fn synchronize(&self, p: &mut Parser);
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
        let mut stmts = Vec::<Stmt>::new();

        while !self.is_end() {
            stmts.push(Stmt::new(&mut self));
        }

        Parse {
            // TODO refactor the code for expr, and what pase returns.
            expr: Expr::Error("TODO".to_string()),
        }
    }

    fn peek(&mut self) -> Option<SyntaxKind> {
        self.lexer.peek()
            .map(|(kind, _)| *kind)
    }

    fn next(&mut self) -> (SyntaxKind, &str) {
        self.lexer.next().unwrap()
    }

    fn consume(&mut self) {
        let _ = self.lexer.next();
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
    pub(crate) expr: Expr,
}

#[cfg(test)]
fn check(input: &str, expr: Expr) {
    let parse = Parser::new(input).parse();
    assert_eq!(parse.expr, expr);
}

#[cfg(test)]
mod tests {
    use super::*;

}
