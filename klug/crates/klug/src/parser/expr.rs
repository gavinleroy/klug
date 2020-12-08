mod literal;
mod op;

use literal::Literal;
use op::{InfixOp, PrefixOp};
use super::Parser;
use crate::lexer::SyntaxKind;

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

    let mut poss_expr: Expr;

    p.consume_whitespace();

    match p.peek() {
        Some(SyntaxKind::Number) 
        | Some(SyntaxKind::Ident) => {
            poss_expr = Expr::Literal(Literal::new(p.next())); // NOTE: get consumes the char
        }
        Some(SyntaxKind::Minus) => {
            p.consume();
            let op = PrefixOp::Neg;
            let ((), rbind) = op.binding_power();
            let new_expr = expr_binding_power(p, rbind);
            poss_expr =  Expr::Unary(op, Box::new(new_expr));
        }
        Some(SyntaxKind::LParen) => {
            p.consume();
            let new_expr = expr_binding_power(p, 0);
            assert_eq!(p.peek(), Some(SyntaxKind::RParen));
            p.consume(); // consume the paren
            poss_expr =  Expr::Grouping(Box::new(new_expr));
        }
        _ => todo!(), // TODO handle errors
    }

    p.consume_whitespace();

    loop {
        let op = match p.peek() {
            Some(SyntaxKind::Plus) => InfixOp::Add,
            Some(SyntaxKind::Minus) => InfixOp::Sub,
            Some(SyntaxKind::Star) => InfixOp::Mul,
            Some(SyntaxKind::Slash) => InfixOp::Div,
            _ => return poss_expr, // If it's not an op, we're done with the expr
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

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::check;

    #[test]
    fn parse_num() {
        check("5", Expr::Literal(Literal::NUMBER(5.0)));
    }

    #[test]
    fn parse_bool() {
        check("false", Expr::Literal(Literal::FALSE));
    }

    #[test]
    fn literal_string() {
        let s = "var";
        assert_eq!(Literal::new(s), Literal::STRING("var".to_string()));
    }

    #[test]
    fn literal_num() {
        let s = "5";
        assert_eq!(Literal::new(s), Literal::NUMBER(5.0));
    }

    #[test]
    fn literal_true() {
        let s = "true";
        assert_eq!(Literal::new(s), Literal::TRUE);
    }

    #[test]
    fn literal_false() {
        let s = "false";
        assert_eq!(Literal::new(s), Literal::FALSE);
    }

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
