use crate::utils;
use crate::stmt::Stmt;

#[derive(Debug, PartialEq)]
pub(crate) struct FuncDef {
    pub(crate) name: String,
    pub(crate) params: Vec<String>,
    pub(crate) body: Box<Stmt>,
}

impl FuncDef {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("fn", s)?;
        let (_, s) = utils::extract_whitespace1(s)?;

        let (name, s) = utils::extract_ident(s)?;
        let (_, s) = utils::extract_whitespace(s);

        println!("fn name: {}, s: {}", name, s);

        let (s, params) = utils::sequence(
            |s| utils::extract_ident(s).map(|(ident, s)| (s, ident.to_string())), 
            s)?;

        let s = utils::tag("=>", s)?;
        let (_, s) = utils::extract_whitespace(s);

        let (s, body) = Stmt::new(s)?;

        Ok((
            s,
            Self {
                name: name.to_string(),
                params: params,
                body: Box::new(body),
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{Block, Expr};

    #[test]
    fn parse_func_def_with_no_params_and_empty_body() {
        assert_eq!(
            FuncDef::new("fn nothing => {}"),
            Ok((
                "",
                FuncDef {
                    name: "nothing".to_string(),
                    params: Vec::new(),
                    body: Box::new(Stmt::Expr(Expr::Block(Block { stmts: Vec::new() }))),
                },
            )),
        );
    }
    #[test]
    fn parse_func_def_with_one_param_and_empty_body() {
        assert_eq!(
            FuncDef::new("fn greet name => {}"),
            Ok((
                "",
                FuncDef {
                    name: "greet".to_string(),
                    params: vec!["name".to_string()],
                    body: Box::new(Stmt::Expr(Expr::Block(Block { stmts: Vec::new() }))),
                },
            )),
        );
    }
    #[test]
    fn parse_func_def_with_multiple_params() {
        assert_eq!(
            FuncDef::new("fn add x y => x + y"),
            Ok((
                "",
                FuncDef {
                    name: "add".to_string(),
                    params: vec!["x".to_string(), "y".to_string()],
                    body: Box::new(Stmt::Expr(Expr::Operation {
                        lhs: Expr::BindingUsage(BindingUsage {
                            name: "x".to_string()
                        }),
                        rhs: Expr::BindingUsage(BindingUsage {
                            name: "y".to_string()
                        }),
                        op: Op::Add
                    }))
                }
            ))
        );
    }
}
