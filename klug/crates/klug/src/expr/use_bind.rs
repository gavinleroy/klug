use crate::utils;
use crate::value::Value;
use crate::env::Env;

#[derive(Debug, PartialEq)]
pub(crate) struct BindingUsage {
    pub(crate) name: String,
}

impl BindingUsage {
    pub(super) fn new(s: &str) -> Result<(&str, Self), String> {
        let (name, s) = utils::extract_ident(s)?;

        Ok((s, Self { name: name.to_string() } ))
    }
    pub(super) fn eval(&self, env: &Env) -> Result<Value, String> {
        env.lookup(&self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_binding_usage() {
        assert_eq!(
            BindingUsage::new("abc"),
            Ok((
                "",
                BindingUsage {
                    name: "abc".to_string(),
                },
            )),
        );
    }
        #[test]
    fn eval_existing_binding_usage() {
        let mut env = Env::default();
        env.extend_env("foo".to_string(), Value::Number(10));

        assert_eq!(
            BindingUsage {
                name: "foo".to_string(),
            }
            .eval(&env),
            Ok(Value::Number(10)),
        );
    }
}
