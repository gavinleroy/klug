use std::fmt;
use super::expr::Expr;
use super::Parser;
use crate::lexer::SyntaxKind;

#[derive(Debug, PartialEq)]
pub(crate) enum Stmt {
    Expr(Box<Expr>),
    Error(String),
}

impl Stmt {
    pub(super) fn new(p: &mut Parser) -> Self {
        if let Ok(stmt) = new_stmt(p) {
            stmt
        } else {
            Self::Error("TODO".to_string()) // TODO actually report the error
        }
    }
}

fn new_stmt(p: &mut Parser) -> Result<Stmt, Stmt>  {
    // TODO actually parse statements
//    match p.peek() {
//    }
    Ok( Stmt::Expr(Box::new(Expr::new(p))) )
}
