use std::collections::HashMap;
use crate::value::Value;

#[derive(Debug, PartialEq, Default)]
pub(crate) struct Env {
    bindings: HashMap<String, Value>,
}

impl Env {
    pub(crate) fn extend_env(&mut self, name: String, val: Value) {
        self.bindings.insert(name, val);
    }
}
