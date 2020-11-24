mod utils;

#[derive(Debug, PartialEq)]
pub struct Expr {
    pub lhs: Number,
    pub rhs: Number,
    pub op: Op,
}

impl Expr {
    pub fn new(s: &str) -> Self {
        Self {
            lhs: Number(1),
            rhs: Number(2),
            op: Op::Plus
        }
    }
}


#[derive(Debug, PartialEq)]
pub struct Number(i32);

impl Number {
    pub fn new(s: &str) -> Self {
       Self(s.parse().unwrap()) 
    }
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Plus,
    Mult,
    Sub,
    Div,
}

impl Op {
    pub fn new(s: &str) -> Self {
        match s {
            "+" => Self::Plus,
            "*" => Self::Mult,
            "-" => Self::Sub,
            "/" => Self::Div,
            _ => panic!("invalid operator"),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), Number(123));
    }
    #[test]
    fn parse_add() {
        assert_eq!(Op::new("+"), Op::Plus);
    }
    #[test]
    fn parse_mult() {
        assert_eq!(Op::new("*"), Op::Mult);
    }
    #[test]
    fn parse_sub() {
        assert_eq!(Op::new("-"), Op::Sub);
    }
    #[test]
    fn parse_div() {
        assert_eq!(Op::new("/"), Op::Div);
    }
    #[test]
    fn parse_expr1plus2() {
        assert_eq!(
            Expr::new("1+2"), 
            Expr { 
                lhs: Number(1), 
                rhs: Number(2), 
                op: Op::Plus, 
            });
    }
}