use crate::utils;
use crate::expr::Expr;
use crate::env::Env;

#[derive(Debug, PartialEq)]
pub struct BindingDef {
    name: String,
    val: Expr,
}

impl BindingDef {
    pub fn new(s: &str) -> (&str, Self) {
        let s = utils::tag("let", s);
        let (_, s) = utils::extract_whitespace(s);

        let (name, s) = utils::extract_ident(s);
        let (_, s) = utils::extract_whitespace(s);

        let s = utils::tag("=", s);
        let (_, s) = utils::extract_whitespace(s);

        let (s, val) = Expr::new(s);

        (s, Self { name: name.to_string(), val, } )
    }
    pub(crate) fn eval(&self, env: &mut Env) {
        env.extend_env(self.name.clone(), self.val.eval());
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
            (
                "",
                BindingDef {
                    name: "a".to_string(),
                    val: Expr {
                        lhs: Number(10),
                        rhs: Number(2),
                        op: Op::Div,
                    },
                },
            ),
        );
    }
}
