use std::collections::HashMap;
use crate::value::Value;
use crate::stmt::Stmt;
use crate::expr::use_bind::BindingUsage;

#[derive(Debug, PartialEq, Default)]
pub struct Env<'parent> {
    bindings: HashMap<String, BoundInfo>,
    parent: Option<&'parent Self>,
}

#[derive(Debug, PartialEq, Clone)]
enum BoundInfo {
    Binding(Value),
    Func { params: Vec<String>, bdy: Stmt },
}

impl BoundInfo {
    fn into_binding(self) -> Option<Value> {
        if let Self::Binding(val) = self {
            Some(val)
        } else {
            None
        }
    }
    fn into_func(self) -> Option<(Vec<String>, Stmt)> {
        if let Self::Func { params, bdy } = self {
            Some((params, bdy))
        } else {
            None
        }
    }
}

impl<'parent> Env<'parent> {
    pub(crate) fn extend_env(&mut self, name: String, val: Value) {
        self.bindings.insert(name, BoundInfo::Binding(val));
    }

    pub(crate) fn extend_env_func(&mut self, name: String, params: Vec<String>, bdy: Stmt) {
        self.bindings.insert(name, BoundInfo::Func{ params: params, bdy: bdy });
    }

    pub(crate) fn create_child(&'parent self) -> Self {
        Self {
            bindings: HashMap::new(),
            parent: Some(self),
        }
    }

    pub(crate) fn get_binding(&self, name: &str) -> Result<Value, String> {
        self.get_bound_info(name)
            .and_then(BoundInfo::into_binding)
            .ok_or_else(|| format!("{} not bound", name))
    }

    pub(crate) fn get_func(&self, name: &str) -> Result<(Vec<String>, Stmt), String> {
        self.get_bound_info(name)
            .and_then(BoundInfo::into_func)
            .ok_or_else(|| format!("{} not bound", name))
    }

    fn get_bound_info(&self, name: &str) -> Option<BoundInfo> {
        self.bindings
            .get(name)
            .cloned()
            .or_else(|| {
            self.parent
                .and_then(|parent| parent.get_bound_info(name))
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
