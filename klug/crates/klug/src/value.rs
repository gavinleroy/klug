use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(i32),
    Unit,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{}", n),
            Self::Unit => write!(f, "Unit"),
        }
    }
}
