use crate::lexer::SyntaxKind;

#[derive(Debug, PartialEq)]
pub(crate) enum Literal {
    NUMBER(f64),
    IDENT(String),
    STRING(String),
    TRUE,
    FALSE,
}

// create a new literal from a string
impl Literal {
    pub(crate) fn new(sk: SyntaxKind, s: &str) -> Self {
        match sk {
            SyntaxKind::TrueKw => Self::TRUE,
            SyntaxKind::FalseKw => Self::FALSE,
            SyntaxKind::Ident => Self::IDENT(s.to_string()),
            SyntaxKind::Number => Self::NUMBER(s.parse::<f64>().unwrap()),
            // NOTE strip off the '...' wrapping the string
            SyntaxKind::StringKw => Self::STRING(s[1..s.len()-1].to_string()),
            _ => unreachable!(),
        }
    }

    pub(super) fn stringify(&self) -> String {
        match self {
           Self::NUMBER(n) => n.to_string(),
           Self::IDENT(s) => (*s).clone(),
           Self::STRING(s) => (*s).clone(),
           Self::TRUE => "true".to_string(),
           Self::FALSE => "false".to_string(),
        }
    }
}

