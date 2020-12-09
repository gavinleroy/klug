pub mod value;

use std::fmt;
use num_traits::Float;
use float_cmp::approx_eq;
use value::Value;
use crate::parser::expr::{
    Expr,
    literal::Literal,
    op::{InfixOp, PrefixOp}
};

#[derive(Debug)]
pub(crate) struct RuntimeError {
    expr: Expr,
    msg: String,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EXPR: {}\nMSG: {}", self.expr.stringify(), self.msg)
    }    
}

// interp - I'm going for a fully functional style here :)
pub(crate) fn interp(expr: Expr) -> Result<Value, RuntimeError> {
    match expr {
        // terminals
        Expr::Literal(Literal::NUMBER(num)) => Ok(Value::Number(num)),
        Expr::Literal(Literal::IDENT(_)) => todo!(),
        Expr::Literal(Literal::STRING(s)) => todo!(),
        Expr::Literal(Literal::TRUE) => Ok(Value::Bool(true)),
        Expr::Literal(Literal::FALSE) => Ok(Value::Bool(false)),

        Expr::Unary(prefop, box_bdy) => {
            let val = interp(*box_bdy)?;
            match (prefop, val) {
                (PrefixOp::Not, Value::Bool(b)) => Ok(Value::Bool(!b)),
                (PrefixOp::Neg, Value::Number(n)) => Ok(Value::Number(-n)),
                _ => todo!(),
            }
        }

        Expr::Binary(box_lhs, infop, box_rhs) => {
            let vlhs = interp(*box_lhs)?;
            let vrhs = interp(*box_rhs)?;
            match (vlhs, vrhs) {
                (Value::Number(n1), Value::Number(n2)) => NumCalc(n1, n2, infop),
                (Value::Str(s1), Value::Str(s2)) => todo!(),
                (Value::Bool(b1), Value::Bool(b2)) => todo!(),
                _ => todo!(),
            }
        }

        Expr::Grouping(box_bdy) => interp(*box_bdy),

        _ => unreachable!(), // I'll handle errors later
    }
}

//fn NumCalc<T: Float>(n1: T, n2: T, op: InfixOp) -> Value {
fn NumCalc(n1: f64, n2: f64, op: InfixOp) -> Result<Value, RuntimeError>{
    let num = match op {
       InfixOp::Add => n1 + n2, 
       InfixOp::Mul => n1 * n2, 
       InfixOp::Sub => n1 - n2, 
       InfixOp::Div if approx_eq!(f64, n2, 0.0) => panic!("division by 0"),
       InfixOp::Div => n1 / n2,
    };
    Ok(Value::Number(num))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interp_num() {
        assert_eq!(interp(Expr::Literal(Literal::NUMBER(5.0))).unwrap(), Value::Number(5.0));
    }

    #[test]
    fn interp_bool() {
        assert_eq!(
            interp(Expr::Literal(Literal::FALSE)).unwrap(), 
            Value::Bool(false));
    }

    #[test]
    fn simple_binary() {
        assert_eq!(
            interp(Expr::Binary(Box::new(Expr::Literal(Literal::NUMBER(1.0))), 
                           InfixOp::Add, 
                           Box::new(Expr::Literal(Literal::NUMBER(2.0))))).unwrap(), 
            Value::Number(3.0));  
    }

    #[test]
    fn simple_compound() {
        assert_eq!(
            interp(Expr::Binary(
                  Box::new(Expr::Binary(
                      Box::new(Expr::Literal(Literal::NUMBER(1.0))),
                      InfixOp::Add, 
                      Box::new(Expr::Literal(Literal::NUMBER(2.0))))),
                  InfixOp::Sub, 
                  Box::new(Expr::Literal(Literal::NUMBER(4.0))))).unwrap(), 
            Value::Number(-1.0));
    }

    #[test]
    fn simple_precedence1() {
        assert_eq!(
            interp(Expr::Binary(
                  Box::new(Expr::Literal(Literal::NUMBER(1.0))),
                  InfixOp::Add, 
                  Box::new(Expr::Binary(
                      Box::new(Expr::Literal(Literal::NUMBER(2.0))),
                      InfixOp::Mul, 
                      Box::new(Expr::Literal(Literal::NUMBER(4.0))))))).unwrap(), 
            Value::Number(9.0));
    }

    #[test]
    fn simple_precedence2() {
        assert_eq!(
            interp(Expr::Binary(
                Box::new(Expr::Binary(
                    Box::new(Expr::Literal(Literal::NUMBER(1.0))),
                    InfixOp::Mul,
                    Box::new(Expr::Literal(Literal::NUMBER(2.0))))),
                InfixOp::Sub,
                Box::new(Expr::Literal(Literal::NUMBER(4.0))))).unwrap(), 
            Value::Number(-2.0));
    }

    // rough ...
    #[test]
    fn simple_grouping() {
        assert_eq!(
            interp(Expr::Binary(
                  Box::new(Expr::Binary(
                          Box::new(Expr::Literal(Literal::NUMBER(1.0))),
                          InfixOp::Mul,
                          Box::new(Expr::Grouping(
                                  Box::new(Expr::Binary(
                                          Box::new(Expr::Literal(Literal::NUMBER(2.0))),
                                          InfixOp::Add,
                                          Box::new(Expr::Literal(Literal::NUMBER(2.0))))))))),
                  InfixOp::Div,
                  Box::new(Expr::Literal(Literal::NUMBER(4.0))))).unwrap(), 
            Value::Number(1.0));
    }

    #[test]
    fn unary_expr_minus() {
        assert_eq!(
            interp(Expr::Binary(
                  Box::new(Expr::Unary(PrefixOp::Neg, Box::new(Expr::Literal(Literal::NUMBER(10.0))))),
                  InfixOp::Add,
                  Box::new(Expr::Literal(Literal::NUMBER(20.0))))).unwrap(), 
            Value::Number(10.0));
    }
    #[test]
    fn unary_expr_bang() {
        assert_eq!(
              interp(Expr::Unary(PrefixOp::Not, Box::new(Expr::Literal(Literal::TRUE)))).unwrap(),
              Value::Bool(false));
    }
}
