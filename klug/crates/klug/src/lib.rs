mod parser;
mod lexer;
mod syntax;
mod interp;

use interp::{interp_stmt, value::Value};
use parser::Parser;

pub fn run(input: &str) -> String {
    let parse = Parser::new(input).parse();
    let mut v = "null".to_string();
    for stmt in parse.stmts {
        v = match interp_stmt(stmt) {
            Ok(Value::Str(s)) => s.to_string(),
            Ok(Value::Number(n)) => n.to_string(),
            _ => "TODO".to_string(),
        };
        return v;
    }
    v
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
