#[derive(Debug, PartialEq)]
pub(crate) enum Literal {
    NUMBER(f64),
    STRING(String),
    TRUE,
    FALSE,
}

// create a new literal from a string
impl Literal {
    pub(crate) fn new(s: &str) -> Self {
        if s == "true" {
            Self::TRUE
        } else if s == "false" {
            Self::FALSE
        } else if let Ok(num) = s.parse() {
            Self::NUMBER(num)
        } else {
            // default to making it a variable
            Self::STRING(s.to_string())
        }
    }
}

