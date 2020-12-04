use super::Expr;
use crate::utils;
use crate::env::Env;
use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FuncCall {
    pub(crate) callee: String,
    pub(crate) params: Vec<Expr>,
}

impl FuncCall {
    pub(super) fn new(s: &str) -> Result<(&str, Self), String> {
        let (callee, s) = utils::extract_ident(s)?;
        let (_, s) = utils::take_while(|c| c==' ', s);

        let (s, params) = utils::sequence1(Expr::new, 
                                           |s| utils::take_while(|c| c==' ', s),
                                           s)?;
        Ok((
            s,
            Self {
                callee: callee.to_string(),
                params: params,
            },
        ))
    }

    pub(super) fn eval(&self, env: &Env) -> Result<Value, String> {
        let mut child_env = env.create_child();
        let (ns, bdy) = env.get_func(&self.callee)?;

        let num_expected_params = ns.len();
        let num_actual_params = self.params.len();

        if num_expected_params != num_actual_params {
            return Err("wrong arity".to_string());
        }

        for (param_name, param_expr) in ns.into_iter().zip(&self.params) {
            let param_val = param_expr.eval(&child_env)?;
            child_env.extend_env(param_name, param_val);
        }
        bdy.eval(&mut child_env)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{BindingUsage, Number, Op};
    use crate::stmt::Stmt;
    use super::*;

    #[test]
    fn parse_func_call_with_one_parameter() {
        assert_eq!(
            FuncCall::new("factorial 10"),
            Ok((
                "",
                FuncCall {
                    callee: "factorial".to_string(),
                    params: vec![Expr::Number(Number(10))],
                },
            )),
        );
    }
    #[test]
    fn eval_func_call_with_too_few_parameters() {
        let mut env = Env::default();

        env.extend_env_func(
            "mul".to_string(),
            vec!["a".to_string(), "b".to_string()],
            Stmt::Expr(Expr::Operation {
                lhs: Box::new(Expr::BindingUsage(BindingUsage {
                    name: "a".to_string(),
                })),
                rhs: Box::new(Expr::BindingUsage(BindingUsage {
                    name: "b".to_string(),
                })),
                op: Op::Mult,
            }),
        );

        assert_eq!(
            FuncCall {
                callee: "mul".to_string(),
                params: vec![Expr::Number(Number(100))],
            }
            .eval(&env),
            Err("wrong arity".to_string()),
        );
    }

    #[test]
    fn eval_func_call_with_too_many_parameters() {
        let mut env = Env::default();

        env.extend_env_func(
            "square".to_string(),
            vec!["n".to_string()],
            Stmt::Expr(Expr::Operation {
                lhs: Box::new(Expr::BindingUsage(BindingUsage {
                    name: "n".to_string(),
                })),
                rhs: Box::new(Expr::BindingUsage(BindingUsage {
                    name: "n".to_string(),
                })),
                op: Op::Mult,
            }),
        );

        assert_eq!(
            FuncCall {
                callee: "square".to_string(),
                params: vec![Expr::Number(Number(5)), Expr::Number(Number(42))],
            }
            .eval(&env),
            Err("wrong arity".to_string()),
        );
    }
}
