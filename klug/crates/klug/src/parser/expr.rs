pub(crate) mod op;

use std::fmt;
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
  Error(String),
}

impl Expr {
    pub(super) fn new(p: &mut Parser) -> Self {
        expr_binding_power(p, 0)
            .map_or_else(|e|{
                p.error_occurred(&e);
                Self::Error("TODO".to_string())
//                e
            }, |v| v)
    }

    pub(crate) fn stringify(&self) -> String {
        match self {
            Self::Unary(op, bdy) => format!("{}{}", &(op.stringify())[..], &(*bdy.stringify())[..]), 
            Self::Binary(lhs, op, rhs) => {
                format!("{} {} {}", &(*lhs.stringify())[..], &(op.stringify())[..], &(*rhs.stringify())[..])
            }
            Self::Grouping(bdy) => format!("( {} )", &(*bdy.stringify())[..]), 
            Self::Literal(lit) => lit.stringify(), 
            Self::Error(msg) => msg.to_string(),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.stringify())
    }    
}

fn expr_binding_power(p: &mut Parser, min_bind: u8) -> Result<Expr, ParseError> {

    let mut poss_expr: Expr;

    match p.peek() {
        Some(SyntaxKind::Number) 
        | Some(SyntaxKind::StringKw)
        | Some(SyntaxKind::TrueKw)
        | Some(SyntaxKind::FalseKw)
        | Some(SyntaxKind::Ident) => {
            // NOTE: next consumes the token
            let (sk, txt) = p.next();
            poss_expr = Expr::Literal(Literal::new(sk, txt)); 
        }
        Some(SyntaxKind::Minus) 
        | Some(SyntaxKind::Bang) => {
            let (kind, _) = p.next();
            let op = PrefixOp::from_kind(kind);
            let ((), rbind) = op.binding_power();
            let new_expr = expr_binding_power(p, rbind)?;
            poss_expr =  Expr::Unary(op, Box::new(new_expr));
        }
        Some(SyntaxKind::LParen) => {
            p.consume();
            let new_expr = expr_binding_power(p, 0)?;
            p.expect(SyntaxKind::RParen)?;
            poss_expr =  Expr::Grouping(Box::new(new_expr));
        }
        _ => {
            let(_, txt) = p.next();
            return Err(Expr::Error(format!("expecting expr token, received {}", txt))); 
        }
    }

    loop {
        let op = match p.peek() {
            Some(SyntaxKind::Plus) => InfixOp::Add,
            Some(SyntaxKind::Minus) => InfixOp::Sub,
            Some(SyntaxKind::Star) => InfixOp::Mul,
            Some(SyntaxKind::Slash) => InfixOp::Div,
            _ => return Ok(poss_expr), // If it's not an op, we're done with the expr
        };

        let (lbind, rbind) = op.binding_power();

        // preceding expr takes precedence
        if lbind < min_bind {
            return Ok(poss_expr);
        }

        p.consume(); // consume the operator token
        let rhs = expr_binding_power(p, rbind)?;
        poss_expr = Expr::Binary(Box::new(poss_expr), op, Box::new(rhs));
    }
}

// fn expect(sk: SyntaxKind, p: &mut Parser) -> Result<String, Expr> {
//     if p.peek() != Some(sk) {
//         return Err(Expr::Error(format!("Expected {:?}", sk)))
//     } else {
//         let (_, txt) = p.next();
//         Ok(txt)
//     }
// }

#[cfg(test)]
fn check(input: &str, to_check: Expr) {
    let expr = Expr::new(&mut Parser::new(input));
    assert_eq!(expr, to_check);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_num() {
        check("5", Expr::Literal(Literal::NUMBER(5.0)));
    }

    #[test]
    fn parse_bool() {
        check("false", Expr::Literal(Literal::FALSE));
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

    #[test]
    fn unary_expr_minus() {
        check("-10 + 20", 
              Expr::Binary(
                  Box::new(Expr::Unary(PrefixOp::Neg, Box::new(Expr::Literal(Literal::NUMBER(10.0))))),
                  InfixOp::Add,
                  Box::new(Expr::Literal(Literal::NUMBER(20.0)))));
    }
    #[test]
    fn unary_expr_bang() {
        check("!true + false", 
              Expr::Binary(
                  Box::new(Expr::Unary(PrefixOp::Not, Box::new(Expr::Literal(Literal::TRUE)))),
                  InfixOp::Add,
                  Box::new(Expr::Literal(Literal::FALSE))));
    }
}
