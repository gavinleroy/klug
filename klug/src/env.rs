use std::collections::HashMap;
use crate::value::Value;
use crate::use_bind::BindingUsage;

#[derive(Debug, PartialEq, Default)]
pub(crate) struct Env {
    bindings: HashMap<String, Value>,
}

impl Env {
    pub(crate) fn extend_env(&mut self, name: String, val: Value) {
        self.bindings.insert(name, val);
    }
    pub(crate) fn lookup(&self, name: &str) -> Result<Value, String> {
        self.bindings
            .get(name)
            .cloned()
            .ok_or_else(|| format!("{} is not bound", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_non_existent_binding_usage() {
        let empty_env = Env::default();

        assert_eq!(
            BindingUsage {
                name: "i_dont_exist".to_string(),
            }
            .eval(&empty_env),
            Err("i_dont_exist is not bound".to_string()),
        );
    }
}
