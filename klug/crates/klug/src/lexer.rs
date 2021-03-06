use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, Copy, Clone, PartialEq, Logos, FromPrimitive, ToPrimitive)]
pub(crate) enum SyntaxKind {
    // NOTE I don't care about whitespace
    #[regex(r"[ \t\f]+", logos::skip)]
    Whitespace,
    // NOTE I do care about newlines though :)
    #[regex(r"\n")]
    Newline,

    #[token("fn")]
    FnKw,
    #[token("let")]
    LetKw,
    #[token("true")]
    TrueKw,
    #[token("false")]
    FalseKw,
    #[regex(r"[_a-zA-Z][\w]*")]
    Ident,
    #[regex(r"[\d]+")]
    Number,
    // NOTE strings in Klug are written like this: 'string'
    #[regex(r"'([^']*)'")]
    StringKw,

    #[token("<=")] // NOTE these aren't incorporated into the expr module yet
    LessThanEq,
    #[token(">=")]
    GreaterThanEq,
    #[token("<")]
    LessThan,
    #[token(">")]
    GreaterThan,
    #[token("!=")]
    NotEqual,
    #[token("==")]
    Equal,
    #[token("&")]
    BitAnd,
    #[token("|")]
    BitOr,

    #[token("!")]
    Bang,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("=")]
    Equals,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    Root, // TODO remove
    #[error]
    Error,
}

pub(crate) struct Lexer<'a> {
    inner: logos::Lexer<'a, SyntaxKind>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            inner: SyntaxKind::lexer(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (SyntaxKind, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let text = self.inner.slice();

        Some((kind, text))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, kind: SyntaxKind) {
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next(), Some((kind, input)));
    }

    #[test]
    fn lex_spaces() {
        let mut lexer = Lexer::new("        ");
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn lex_fn_keyword() {
        check("fn", SyntaxKind::FnKw);
    }
    #[test]
    fn lex_let_keyword() {
        check("let", SyntaxKind::LetKw);
    }

    #[test]
    fn lex_alphabetic_identifier() {
        check("abcd", SyntaxKind::Ident);
    }

    #[test]
    fn lex_alphanumeric_identifier() {
        check("ab123cde456", SyntaxKind::Ident);
    }

    #[test]
    fn lex_mixed_case_identifier() {
        check("ABCdef", SyntaxKind::Ident);
    }

    #[test]
    fn lex_number() {
        check("123456", SyntaxKind::Number);
    }

    #[test]
    fn lex_plus() {
        check("+", SyntaxKind::Plus);
    }

    #[test]
    fn lex_minus() {
        check("-", SyntaxKind::Minus);
    }

    #[test]
    fn lex_star() {
        check("*", SyntaxKind::Star);
    }

    #[test]
    fn lex_slash() {
        check("/", SyntaxKind::Slash);
    }

    #[test]
    fn lex_equals() {
        check("=", SyntaxKind::Equals);
    }

    #[test]
    fn lex_left_brace() {
        check("{", SyntaxKind::LBrace);
    }

    #[test]
    fn lex_right_brace() {
        check("}", SyntaxKind::RBrace);
    }

    #[test]
    fn lex_single_char_identifier() {
        check("x", SyntaxKind::Ident);
    }

    #[test]
    fn lex_left_parenthesis() {
        check("(", SyntaxKind::LParen);
    }

    #[test]
    fn lex_right_parenthesis() {
        check(")", SyntaxKind::RParen);
    }

    #[test]
    fn match_string1() {
        check("'hello world'", SyntaxKind::StringKw);
    }

    #[test]
    fn match_string2() {
        check("'     hello world 0980291212   \n\n\t\t\t\t        '", SyntaxKind::StringKw);
    }
}
