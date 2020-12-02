use crate::binding_def::BindingDef;
use crate::expr::Expr;
use crate::value::Value;
use crate::env::Env;
use crate::func_def::FuncDef;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Stmt {
    BindingDef(BindingDef),
    FuncDef(FuncDef),
    Expr(Expr),
}

impl Stmt {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        BindingDef::new(s)
            .map(|(s, bind_def)| (s, Self::BindingDef(bind_def)))
            .or_else(|_| FuncDef::new(s).map(|(s, fd)| (s, Self::FuncDef(fd))))
            .or_else(|_| Expr::new(s).map(|(s, expr)| (s, Self::Expr(expr))))
    }
    pub(crate) fn eval(&self, env: &mut Env) -> Result<Value, String> {
            match self {
            Self::BindingDef(binding_def) => {
                binding_def.eval(env)?;
                Ok(Value::Unit)
            }
            Self::Expr(expr) => expr.eval(env),
            Self::FuncDef(fd) => {
                fd.eval(env)?; 
                Ok(Value::Unit)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{BindingUsage, Number, Op};

    #[test]
    fn parse_func_def() {
        assert_eq!(
            Stmt::new("fn identity x => x"),
            Ok((
                "",
                Stmt::FuncDef(FuncDef {
                    name: "identity".to_string(),
                    params: vec!["x".to_string()],
                    body: Box::new(Stmt::Expr(Expr::BindingUsage(BindingUsage {
                        name: "x".to_string(),
                    }))),
                }),
            )),
        );
    }
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
                    lhs: Box::new(Expr::Number(Number(1))),
                    rhs: Box::new(Expr::Number(Number(1))),
                    op: Op::Plus,
                }),
            )),
        );
    }
    #[test]
    fn eval_func_def() {
        assert_eq!(
            Stmt::FuncDef(FuncDef {
                name: "always_return_one".to_string(),
                params: Vec::new(),
                body: Box::new(Stmt::Expr(Expr::Number(Number(1)))),
            })
            .eval(&mut Env::default()),
            Ok(Value::Unit),
        );
    }
}
