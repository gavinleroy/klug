use super::Parser;
use crate::lexer::SyntaxKind;
use crate::literal::{Literal, new_literal};

#[derive(Debug, PartialEq)]
pub(crate) enum Expr{
  Unary(PrefixOp, Box<Expr>),
  Binary(Box<Expr>, InfixOp, Box<Expr>),
  Grouping(Box<Expr>),
  Literal(Literal),
}

impl Expr {
    pub(super) fn new(p: &mut Parser) -> Expr {
        expr_binding_power(p, 0)
    }
}

fn expr_binding_power(p: &mut Parser, min_bind: u8) -> Expr {
//    let checkpoint = p.checkpoint();

    let mut poss_expr: Expr;

    p.consume_whitespace();

    match p.peek() {
        Some(SyntaxKind::Number) 
        | Some(SyntaxKind::Ident) => {
            // start of an binary expr
            poss_expr = Expr::Literal(new_literal(p.get()));
        }
        Some(SyntaxKind::Minus) => {
            // start of a unary -
            p.consume();
            let op = PrefixOp::Neg;
            let ((), rbind) = op.binding_power();
            let new_expr = expr_binding_power(p, rbind);
            poss_expr =  Expr::Unary(op, Box::new(new_expr));
        }
        Some(SyntaxKind::LParen) => {
            // start of a grouping
            p.consume();
            let new_expr = expr_binding_power(p, 0);
            assert_eq!(p.peek(), Some(SyntaxKind::RParen));
            p.consume(); // consume the paren
            poss_expr =  Expr::Grouping(Box::new(new_expr));
        }
        _ => todo!(),
    }

    p.consume_whitespace();

    loop {
        let op = match p.peek() {
            Some(SyntaxKind::Plus) => InfixOp::Add,
            Some(SyntaxKind::Minus) => InfixOp::Sub,
            Some(SyntaxKind::Star) => InfixOp::Mul,
            Some(SyntaxKind::Slash) => InfixOp::Div,
            _ => return poss_expr, // I’ll handle errors later.
        };

        let (lbind, rbind) = op.binding_power();

        // preceding expr takes precedence
        if lbind < min_bind {
            return poss_expr;
        }

        p.consume(); // consume the operator token
        let rhs = expr_binding_power(p, rbind);
        poss_expr = Expr::Binary(Box::new(poss_expr), op, Box::new(rhs));
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum InfixOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl InfixOp {
    fn binding_power(&self) -> (u8, u8) {
        match self {
            Self::Add | Self::Sub => (1, 2),
            Self::Mul | Self::Div => (3, 4),
        }
    }
}


#[derive(Debug, PartialEq)]
pub(crate) enum PrefixOp {
    Neg,
}

impl PrefixOp {
    fn binding_power(&self) -> ((), u8) {
        match self {
            Self::Neg => ((), 5),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::check;
    use super::*;

    #[test]
    fn simple_binary() {
        check("1 + 2", 
              Expr::Binary(Box::new(Expr::Literal(Literal::NUMBER(1.0))), 
                           InfixOp::Add, 
                           Box::new(Expr::Literal(Literal::NUMBER(2.0)))));  
    }

    #[test]
    fn simple_compound() {
        check("1 + 2 - 4", 
              Expr::Binary(
                  Box::new(Expr::Binary(
                      Box::new(Expr::Literal(Literal::NUMBER(1.0))),
                      InfixOp::Add, 
                      Box::new(Expr::Literal(Literal::NUMBER(2.0))))),
                  InfixOp::Sub, 
                  Box::new(Expr::Literal(Literal::NUMBER(4.0)))));
    }

    #[test]
    fn simple_precedence1() {
        check("1 + 2 * 4", 
              Expr::Binary(
                  Box::new(Expr::Literal(Literal::NUMBER(1.0))),
                  InfixOp::Add, 
                  Box::new(Expr::Binary(
                      Box::new(Expr::Literal(Literal::NUMBER(2.0))),
                      InfixOp::Mul, 
                      Box::new(Expr::Literal(Literal::NUMBER(4.0)))))));
    }

    #[test]
    fn simple_precedence2() {
        check("1 * 2 - 4", 
            Expr::Binary(
                Box::new(Expr::Binary(
                    Box::new(Expr::Literal(Literal::NUMBER(1.0))),
                    InfixOp::Mul,
                    Box::new(Expr::Literal(Literal::NUMBER(2.0))))),
                InfixOp::Sub,
                Box::new(Expr::Literal(Literal::NUMBER(4.0)))));
    }

    // rough ...
    #[test]
    fn simple_grouping() {
        check("1 * (2 + 2) / 4", 
              Expr::Binary(
                  Box::new(Expr::Binary(
                          Box::new(Expr::Literal(Literal::NUMBER(1.0))),
                          InfixOp::Mul,
                          Box::new(Expr::Grouping(
                                  Box::new(Expr::Binary(
                                          Box::new(Expr::Literal(Literal::NUMBER(2.0))),
                                          InfixOp::Add,
                                          Box::new(Expr::Literal(Literal::NUMBER(2.0))))))))),
                  InfixOp::Div,
                  Box::new(Expr::Literal(Literal::NUMBER(4.0)))));
    }
}
