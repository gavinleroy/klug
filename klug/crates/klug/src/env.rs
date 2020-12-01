use std::collections::HashMap;
use crate::value::Value;
use crate::expr::use_bind::BindingUsage;

#[derive(Debug, PartialEq, Default)]
pub struct Env<'parent> {
    bindings: HashMap<String, Value>,
    parent: Option<&'parent Self>,
}

impl<'parent> Env<'parent> {
    pub(crate) fn extend_env(&mut self, name: String, val: Value) {
        self.bindings.insert(name, val);
    }
    pub(crate) fn create_child(&'parent self) -> Self {
        Self {
            bindings: HashMap::new(),
            parent: Some(self),
        }
    }
    pub(crate) fn lookup(&self, name: &str) -> Result<Value, String> {
        self.lookup_without_error_msg(name)
            .ok_or_else(|| format!("{} is not bound", name))
    }

    fn lookup_without_error_msg(&self, name: &str) -> Option<Value> {
        self.bindings.get(name).cloned().or_else(|| {
            self.parent
                .and_then(|parent| parent.lookup_without_error_msg(name))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

//    #[test]
//    fn eval_non_existent_binding_usage() {
//        let empty_env = Env::default();
//
//        assert_eq!(
//            BindingUsage {
//                name: "i_dont_exist".to_string(),
//            }
//            .eval(&empty_env),
//            Err("i_dont_exist is not bound".to_string()),
//        );
//    }
}
