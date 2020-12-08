#[path = "expr.rs"] mod expr;

use expr::Expr;
use logos::Logos;
use std::iter::Peekable;
use crate::lexer::{Lexer, SyntaxKind};
use crate::syntax::KlugLanguage;

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
//    builder: GreenNodeBuilder<'static>,
}

impl<'a> Parser<'a> {
    pub fn new(s: &'a str) -> Self {
        Self { 
            lexer: Lexer::new(s).peekable(),
//            builder: GreenNodeBuilder::new(),
        }
    }

    pub fn parse(mut self) -> Parse {
//        self.start_node(SyntaxKind::Root);

        let new_expr = Expr::new(&mut self);

//        self.finish_node();

        Parse {
            expr: new_expr,
//            green_node: self.builder.finish(),
        }
    }

//    fn start_node_at(&mut self, checkpoint: Checkpoint, kind: SyntaxKind) {
//        self.builder
//            .start_node_at(checkpoint, KlugLanguage::kind_to_raw(kind));
//    }

//    fn checkpoint(&self) -> Checkpoint {
//        self.builder.checkpoint()
//    }

    fn peek(&mut self) -> Option<SyntaxKind> {
        self.lexer.peek().map(|(kind, _)| *kind)
    }

//    fn bump(&mut self) {
//        let (kind, text) = self.lexer.next().unwrap();
//
//        self.builder
//            .token(KlugLanguage::kind_to_raw(kind), text.into());
//    }

    fn get(&mut self) -> &str{
        let (kind, text) = self.lexer.next().unwrap();
        text
    }

    fn consume(&mut self) {
        let _ = self.lexer.next();
    }

    fn consume_whitespace(&mut self) {
        while self.peek() == Some(SyntaxKind::Whitespace) {
            self.consume();
        }
    }
//    fn start_node(&mut self, kind: SyntaxKind) {
//        self.builder.start_node(KlugLanguage::kind_to_raw(kind));
//    }

//    fn finish_node(&mut self) {
//        self.builder.finish_node();
//    }
}

#[derive(Debug)]
pub struct Parse {
    expr: Expr,
}

impl Parse {
//    fn syntax(&self) -> SyntaxNode {
//        SyntaxNode::new_root(self.green_node.clone())
//    }
//
//    pub fn debug_tree(&self) -> String {
//        let syntax_node = SyntaxNode::new_root(self.green_node.clone());
//        let formatted = format!("{:#?}", syntax_node);
//
//        // We cut off the last byte because formatting the SyntaxNode adds on a newline at the end.
//        formatted[0..formatted.len() - 1].to_string()
//    }
}

#[cfg(test)]
fn check(input: &str, expr: Expr) {
    let parse = Parser::new(input).parse();
    assert_eq!(parse.expr, expr);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::expr::{InfixOp, PrefixOp};
    use crate::literal::Literal;
    use expect_test::expect;

    #[test]
    fn parse_num() {
        check("5", Expr::Literal(Literal::NUMBER(5.0)));
    }
}
