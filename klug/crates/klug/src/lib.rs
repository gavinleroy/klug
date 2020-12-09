mod parser;
mod lexer;
mod syntax;
mod interp;

use interp::{interp as finterp, value::Value};
use parser::Parser;

pub fn run(input: &str) -> String {
    let parse = Parser::new(input).parse();
    let v = match finterp(parse.expr) {
        Ok(Value::Number(n)) => n.to_string(),
        Ok(Value::Bool(b)) => b.to_string(),
        Ok(Value::Str(s)) => s.to_string(),
        Err(runtimeE) => format!("{}", runtimeE),
    };
    v
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
