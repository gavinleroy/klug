use crate::utils;
use crate::value::Value;

#[derive(Debug, PartialEq)]
pub struct Expr {
    pub lhs: Number,
    pub rhs: Number,
    pub op: Op,
}

impl Expr {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, lhs) = Number::new(s);
        let (_, s) = utils::extract_whitespace(s);

        let (s, op) = Op::new(s);
        let (_, s) = utils::extract_whitespace(s);

        let (s, rhs) = Number::new(s);

        (s, Self { lhs, rhs, op })
    }
    pub(crate) fn eval(&self) -> Value {
        let Number(lhs) = self.lhs;
        let Number(rhs) = self.rhs;
        let res = match self.op {
            Op::Plus => lhs + rhs,
            Op::Sub => lhs - rhs,
            Op::Mult => lhs * rhs,
            Op::Div => lhs / rhs,
        };
        Value::Number(res)
    }
}

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

impl Number {
    pub fn new(s: &str) -> (&str, Self) {
        let (num, s) = utils::extract_digits(s);
        (s, Self(num.parse().unwrap())) 
    }
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Plus,
    Mult,
    Sub,
    Div,
}

impl Op {
    pub fn new(s: &str) -> (&str, Self) {
        let (op, s) = utils::extract_op(s);
        let op = match op {
            "+" => Self::Plus,
            "*" => Self::Mult,
            "-" => Self::Sub,
            "/" => Self::Div,
            _ => panic!("invalid operator"),
        };
        (s, op)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), ("", Number(123)));
    }
    #[test]
    fn parse_add() {
        assert_eq!(Op::new("+"), ("", Op::Plus));
    }
    #[test]
    fn parse_mult() {
        assert_eq!(Op::new("*"), ("", Op::Mult));
    }
    #[test]
    fn parse_sub() {
        assert_eq!(Op::new("-"), ("", Op::Sub));
    }
    #[test]
    fn parse_div() {
        assert_eq!(Op::new("/"), ("", Op::Div));
    }
    #[test]
    fn parse_expr1plus2() {
        assert_eq!(Expr::new("1+2"), 
                   ("", Expr { lhs: Number(1), rhs: Number(2), op: Op::Plus, }));
    }
    #[test]
    fn parse_expr_with_whitespace() {
        assert_eq!(
            Expr::new("2 * 2"),
            (
                "",
                Expr {
                    lhs: Number(2),
                    rhs: Number(2),
                    op: Op::Mult,
                },
            ),
        );
    }
    #[test]
    fn eval_add() {
        assert_eq!(
            Expr {
                lhs: Number(10),
                rhs: Number(10),
                op: Op::Plus,
            }
            .eval(),
            Value::Number(20),
        );
    }
    #[test]
    fn eval_sub() {
        assert_eq!(
            Expr {
                lhs: Number(1),
                rhs: Number(5),
                op: Op::Sub,
            }
            .eval(),
            Value::Number(-4),
        );
    }
    #[test]
    fn eval_mul() {
        assert_eq!(
            Expr {
                lhs: Number(5),
                rhs: Number(6),
                op: Op::Mult,
            }
            .eval(),
            Value::Number(30),
        );
    }
    #[test]
    fn eval_div() {
        assert_eq!(
            Expr {
                lhs: Number(200),
                rhs: Number(20),
                op: Op::Div,
            }
            .eval(),
            Value::Number(10),
        );
    }
}
