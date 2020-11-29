use crate::binding_def::BindingDef;
use crate::expr::Expr;
use crate::value::Value;
use crate::env::Env;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    BindingDef(BindingDef),
    Expr(Expr),
}

impl Stmt {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        BindingDef::new(s)
            .map(|(s, bind_def)| (s, Self::BindingDef(bind_def)))
            .or_else(|_| Expr::new(s).map(|(s, expr)| (s, Self::Expr(expr))))
    }
    pub(crate) fn eval(&self, env: &mut Env) -> Result<Value, String> {
            match self {
            Self::BindingDef(binding_def) => {
                binding_def.eval(env)?;
                Ok(Value::Unit)
            }
            Self::Expr(expr) => expr.eval(env),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{Number, Op};

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            Stmt::new("let a = 10"),
            Ok((
                "",
                Stmt::BindingDef(BindingDef {
                    name: "a".to_string(),
                    val: Expr::Number(Number(10)),
                }),
            )),
        );
    }

    #[test]
    fn parse_expr() {
        assert_eq!(
            Stmt::new("1+1"),
            Ok((
                "",
                Stmt::Expr(Expr::Operation {
                    lhs: Number(1),
                    rhs: Number(1),
                    op: Op::Plus,
                }),
            )),
        );
    }
}
