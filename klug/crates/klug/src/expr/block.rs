use crate::stmt::Stmt;
use crate::utils;
use crate::value::Value;
use crate::env::Env;

#[derive(Debug, PartialEq)]
pub(crate) struct Block {
    pub(crate) stmts: Vec<Stmt>,
}

impl Block {
    pub(super) fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("{", s)?;
        let (_, s) = utils::extract_whitespace(s);

        let (s, stmts) = utils::sequence(Stmt::new, s)?;

        let (_, s) = utils::extract_whitespace(s);
        let s = utils::tag("}", s)?;

        Ok((s, Block { stmts: stmts }))
    }
    pub(super) fn eval(&self, env: &Env) -> Result<Value, String> {
        if self.stmts.is_empty() {
            return Ok(Value::Unit);
        }

        let mut env = env.create_child();

        let stmts_except_last = &self.stmts[..self.stmts.len() - 1];
        for stmt in stmts_except_last {
            stmt.eval(&mut env)?;
        }

        self.stmts.last().unwrap().eval(&mut env)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Expr, Number, Op, BindingUsage};
    use super::*;
    use crate::binding_def::BindingDef;

    #[test]
    fn parse_empty_block() {
        assert_eq!(Block::new("{}"), Ok(("", Block { stmts: Vec::new() })));
    }
    #[test]
    fn parse_empty_block_with_whitespace() {
        assert_eq!(Block::new("{   }"), Ok(("", Block { stmts: Vec::new() })));
    }
    #[test]
    fn parse_block_with_one_stmt() {
        assert_eq!(
            Block::new("{ 5 }"),
            Ok((
                "",
                Block {
                    stmts: vec![Stmt::Expr(Expr::Number(Number(5)))],
                },
            )),
        );
    }
       #[test]
    fn parse_block_with_multiple_stmts() {
        assert_eq!(
            Block::new(
                "{ let a = 10 
                    let b = a 
                    b 
                }",
            ),
            Ok((
                "",
                Block {
                    stmts: vec![
                        Stmt::BindingDef(BindingDef {
                            name: "a".to_string(),
                            val: Expr::Number(Number(10)),
                        }),
                        Stmt::BindingDef(BindingDef {
                            name: "b".to_string(),
                            val: Expr::BindingUsage(BindingUsage {
                                name: "a".to_string(),
                            }),
                        }),
                        Stmt::Expr(Expr::BindingUsage(BindingUsage {
                            name: "b".to_string(),
                        })),
                    ],
                },
            )),
        );
    } 
    #[test]
    fn eval_empty_block() {
        assert_eq!(
            Block { stmts: Vec::new() }.eval(&Env::default()),
            Ok(Value::Unit),
        );
    }
    #[test]
    fn eval_block_with_multiple_binding_defs() {
        assert_eq!(
            Block {
                stmts: vec![
                    Stmt::BindingDef(BindingDef {
                        name: "foo".to_string(),
                        val: Expr::Number(Number(5)),
                    }),
                    Stmt::BindingDef(BindingDef {
                        name: "bar".to_string(),
                        val: Expr::Number(Number(4)),
                    }),
                    Stmt::BindingDef(BindingDef {
                        name: "baz".to_string(),
                        val: Expr::Number(Number(3)),
                    }),
                ],
            }
            .eval(&Env::default()),
            Ok(Value::Unit),
        );
    }
    #[test]
    fn eval_block_with_multiple_exprs() {
        assert_eq!(
            Block {
                stmts: vec![
                    Stmt::Expr(Expr::Number(Number(100))),
                    Stmt::Expr(Expr::Number(Number(30))),
                    Stmt::Expr(Expr::Operation {
                        lhs: Number(10),
                        rhs: Number(7),
                        op: Op::Sub,
                    }),
                ],
            }
            .eval(&Env::default()),
            Ok(Value::Number(3)),
        );
    }
}
