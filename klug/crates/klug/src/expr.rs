pub mod use_bind;
pub mod block;

pub(crate) use use_bind::BindingUsage;
pub(crate) use block::Block;

use crate::value::Value;
use crate::env::Env;
use crate::utils;

#[derive(Debug, PartialEq)]
pub(crate) enum Expr {
    Number(Number),
    Operation { lhs: Number, rhs: Number, op: Op },
    BindingUsage(BindingUsage),
    Block(Block),
}

impl Expr {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        Self::new_op(s)
            .or_else(|_| Self::new_num(s))
            .or_else(|_| { 
                BindingUsage::new(s)
                .map(|(s, binduse)| (s, Self::BindingUsage(binduse)))
            })
            .or_else(|_| {
                Block::new(s)
                    .map(|(s, blck)| (s, Self::Block(blck)))
            })
    }

    fn new_op(s: &str) -> Result<(&str, Self), String> {
        let (s, lhs) = Number::new(s)?;
        let (_, s) = utils::extract_whitespace(s);

        let (s, op) = Op::new(s)?;
        let (_, s) = utils::extract_whitespace(s);

        let (s, rhs) = Number::new(s)?;

        Ok((s, Self::Operation { lhs, rhs, op }))
    }

    fn new_num(s: &str) -> Result<(&str, Self), String> {
        Number::new(s)
            .map(|(s, num)| (s, Self::Number(num)))
    }

    pub(crate) fn eval(&self, env: &Env) -> Result<Value, String> {
        match self {
            Self::Number(Number(n)) => Ok(Value::Number(*n)),
            Self::Operation { lhs, rhs, op } => {
                let Number(lhs) = lhs;
                let Number(rhs) = rhs;
                let res = match op {
                    Op::Plus => lhs + rhs,
                    Op::Sub => lhs - rhs,
                    Op::Mult => lhs * rhs,
                    Op::Div => lhs / rhs,
                };
                Ok(Value::Number(res))
            },
            Self::BindingUsage(binduse) => binduse.eval(&env),
            Self::Block(block) => block.eval(&env),
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Number(pub(crate) i32);

impl Number {
    fn new(s: &str) -> Result<(&str, Self), String> {
        let (num, s) = utils::extract_digits(s)?;
        Ok((s, Self(num.parse().unwrap()))) 
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum Op {
    Plus,
    Mult,
    Sub,
    Div,
}

impl Op {
    fn new(s: &str) -> Result<(&str, Self), String> {
        utils::tag("+", s)
            .map(|s| (s, Self::Plus))
            .or_else(|_| utils::tag("-", s).map(|s| (s, Self::Sub)))
            .or_else(|_| utils::tag("*", s).map(|s| (s, Self::Mult)))
            .or_else(|_| utils::tag("/", s).map(|s| (s, Self::Div)))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), Ok(("", Number(123))));
    }
    #[test]
    fn parse_add() {
        assert_eq!(Op::new("+"), Ok(("", Op::Plus)));
    }
    #[test]
    fn parse_mult() {
        assert_eq!(Op::new("*"), Ok(("", Op::Mult)));
    }
    #[test]
    fn parse_sub() {
        assert_eq!(Op::new("-"), Ok(("", Op::Sub)));
    }
    #[test]
    fn parse_div() {
        assert_eq!(Op::new("/"), Ok(("", Op::Div)));
    }
    #[test]
    fn parse_expr1plus2() {
        assert_eq!(Expr::new("1+2"), 
                   Ok(("", Expr::Operation { lhs: Number(1), rhs: Number(2), op: Op::Plus, })));
    }
    #[test]
    fn parse_expr_with_whitespace() {
        assert_eq!(
            Expr::new("2 * 2"),
            Ok((
                "",
                Expr::Operation {
                    lhs: Number(2),
                    rhs: Number(2),
                    op: Op::Mult,
                },
            )),
        );
    }
    #[test]
    fn eval_add() {
        assert_eq!(
            Expr::Operation {
                lhs: Number(10),
                rhs: Number(10),
                op: Op::Plus,
            }
            .eval(&Env::default()),
            Ok(Value::Number(20)),
        );
    }
    #[test]
    fn eval_sub() {
        assert_eq!(
            Expr::Operation {
                lhs: Number(1),
                rhs: Number(5),
                op: Op::Sub,
            }
            .eval(&Env::default()),
            Ok(Value::Number(-4)),
        );
    }
    #[test]
    fn eval_mul() {
        assert_eq!(
            Expr::Operation {
                lhs: Number(5),
                rhs: Number(6),
                op: Op::Mult,
            }
            .eval(&Env::default()),
            Ok(Value::Number(30)),
        );
    }
    #[test]
    fn eval_div() {
        assert_eq!(
            Expr::Operation {
                lhs: Number(200),
                rhs: Number(20),
                op: Op::Div,
            }
            .eval(&Env::default()),
            Ok(Value::Number(10)),
        );
    }
    #[test]
    fn parse_number_as_expr() {
        assert_eq!(Expr::new("456"), Ok(("", Expr::Number(Number(456)))));
    }
    #[test]
    fn parse_binding_usage() {
        assert_eq!(
            Expr::new("bar"),
            Ok((
                "",
                Expr::BindingUsage(BindingUsage {
                    name: "bar".to_string(),
                }),
            )),
        );
    }
    #[test]
    fn eval_binding_usage() {
        let mut env = Env::default();
        env.extend_env("ten".to_string(), Value::Number(10));

        assert_eq!(
            Expr::BindingUsage(BindingUsage {
                name: "ten".to_string(),
            })
            .eval(&env),
            Ok(Value::Number(10)),
        );
    }
}
