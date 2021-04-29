use std::fmt;
use crate::lexer::SyntaxKind;
use super::{
    stmt::Stmt, 
    expr::Expr, 
    literal::Literal, 
    Parser
};

#[derive(Debug, PartialEq)]
pub(crate) enum Decl {
    Let(Literal::IDENT, Option<Expr>),
    Stmt(Box<Stmt>),
    Error(String),
}

impl Decl {
    pub(super) fn new(p: &mut Parser) -> Self {
        if let Ok(decl) = new_decl(p) {
            decl
        } else {
            Self::Error("TODO".to_string()) // TODO actually report the error
        }
    }
}

fn new_decl(p: &mut Parser) -> Result<Decl, Decl>  {
    match p.peek() {
        Some(SyntaxKind::LetKw) => {
            p.consume(); // eat the 'let'
            let ident = p.expect(SyntaxKind::Ident);
            let lit_ident = Literal::new(SyntaxKind::Ident, ident);
            let expr = None;
            if p.peek() == Some(SyntaxKind::Equals) {
                // There must be an expression
                expr = Some(Expr::new(p));
            } // o.t. remains None
            p.expect(SyntaxKind::Newline)?;
            Decl::Let(lit_ident, expr)
        }
        _ => Decl::Stmt(Box::new(Stmt::new(p))),
    }
}
