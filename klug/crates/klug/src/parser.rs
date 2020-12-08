mod expr;

use expr::Expr;
use logos::Logos;
use std::iter::Peekable;
use crate::lexer::{Lexer, SyntaxKind};
use crate::syntax::KlugLanguage;

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(s: &'a str) -> Self {
        Self { lexer: Lexer::new(s).peekable(), }
    }

    pub fn parse(mut self) -> Parse {
        // won't always be an expression.
        // eventually I should do a match off the first
        // token and then a FSM from there
        let new_expr = Expr::new(&mut self);

        Parse {
            expr: new_expr,
        }
    }

    fn peek(&mut self) -> Option<SyntaxKind> {
        self.lexer.peek().map(|(kind, _)| *kind)
    }

    fn next(&mut self) -> &str{
        let (kind, text) = self.lexer.next().unwrap();
        text
    }

    fn consume(&mut self) {
        let _ = self.lexer.next();
    }

    fn consume_whitespace(&mut self) {
        while self.peek() == Some(SyntaxKind::Whitespace) {
            self.consume();
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Parse {
    expr: Expr,
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
