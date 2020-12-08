#[derive(Debug, PartialEq)]
pub(crate) enum Literal {
    NUMBER(f64),
    STRING(String),
    TRUE,
    FALSE,
}

// create a new literal from a string
pub(crate) fn new_literal(s: &str) -> Literal {
    if s == "true" {
        Literal::TRUE
    } else if s == "false" {
        Literal::FALSE
    } else if let Ok(num) = s.parse() {
        Literal::NUMBER(num)
    } else {
        Literal::STRING(s.to_string())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_string() {
        let s = "var";
        assert_eq!(new_literal(s), Literal::STRING("var".to_string()));
    }

    #[test]
    fn literal_num() {
        let s = "5";
        assert_eq!(new_literal(s), Literal::NUMBER(5.0));
    }

    #[test]
    fn literal_true() {
        let s = "true";
        assert_eq!(new_literal(s), Literal::TRUE);
    }

    #[test]
    fn literal_false() {
        let s = "false";
        assert_eq!(new_literal(s), Literal::FALSE);
    }
}
