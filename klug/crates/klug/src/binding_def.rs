use crate::utils;
use crate::env::Env;
use crate::expr::Expr;

#[derive(Debug, PartialEq)]
pub(crate) struct BindingDef {
    pub(crate) name: String,
    pub(crate) val: Expr,
}

impl BindingDef {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("let", s)?;
        let (_, s) = utils::extract_whitespace1(s)?;

        let (name, s) = utils::extract_ident(s)?;
        let (_, s) = utils::extract_whitespace(s);

        let s = utils::tag("=", s)?;
        let (_, s) = utils::extract_whitespace(s);

        let (s, val) = Expr::new(s)?;

        Ok((s, Self { name: name.to_string(), val, } ))
    }
    pub(crate) fn eval(&self, env: &mut Env) -> Result<(), String> {
        env.extend_env(self.name.clone(), self.val.eval(env)?);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{Number, Op};

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            BindingDef::new("let a = 10 / 2"),
            Ok((
                "",
                BindingDef {
                    name: "a".to_string(),
                    val: Expr::Operation {
                        lhs: Number(10),
                        rhs: Number(2),
                        op: Op::Div,
                    },
                },
            )),
        );
    }
}
