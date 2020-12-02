mod binding_def;
pub mod env;
mod expr;
mod stmt;
mod utils;
mod value;
mod func_def;

pub use env::Env;
pub use value::Value;

#[derive(Debug)]
pub struct Parse(stmt::Stmt);

impl Parse {
    pub fn eval(&self, env: &mut Env) -> Result<Value, String> {
        self.0.eval(env)
    }
}

pub fn parse(s: &str) -> Result<Parse, String> {
    let (s, stmt) = stmt::Stmt::new(s)?;

    if s.is_empty() {
        Ok(Parse(stmt))
    } else {
        Err(format!("input: '{}' not consumed by parser", s))
    }
}
