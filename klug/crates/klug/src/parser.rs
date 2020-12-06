#[path = "expr.rs"] mod expr;

use expr::expr;
use logos::Logos;
use std::iter::Peekable;
use crate::lexer::{Lexer, SyntaxKind};
use crate::syntax::{KlugLanguage, SyntaxNode};
use rowan::{Checkpoint, GreenNode, GreenNodeBuilder, Language};

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    builder: GreenNodeBuilder<'static>,
}

pub struct Parse {
    green_node: GreenNode,
}

impl<'a> Parser<'a> {
    pub fn new(s: &'a str) -> Self {
        Self { 
            lexer: Lexer::new(s).peekable(),
            builder: GreenNodeBuilder::new(),
        }
    }

    pub fn parse(mut self) -> Parse {
        self.start_node(SyntaxKind::Root);

        expr(&mut self);

        self.finish_node();

        Parse {
            green_node: self.builder.finish(),
        }
    }

    fn start_node_at(&mut self, checkpoint: Checkpoint, kind: SyntaxKind) {
        self.builder
            .start_node_at(checkpoint, KlugLanguage::kind_to_raw(kind));
    }

    fn checkpoint(&self) -> Checkpoint {
        self.builder.checkpoint()
    }

    fn peek(&mut self) -> Option<SyntaxKind> {
        self.lexer.peek().map(|(kind, _)| *kind)
    }

    fn bump(&mut self) {
        let (kind, text) = self.lexer.next().unwrap();

        self.builder
            .token(KlugLanguage::kind_to_raw(kind), text.into());
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        self.builder.start_node(KlugLanguage::kind_to_raw(kind));
    }

    fn finish_node(&mut self) {
        self.builder.finish_node();
    }
}

impl Parse {
    pub fn debug_tree(&self) -> String {
        let syntax_node = SyntaxNode::new_root(self.green_node.clone());
        let formatted = format!("{:#?}", syntax_node);

        // We cut off the last byte because formatting the SyntaxNode adds on a newline at the end.
        formatted[0..formatted.len() - 1].to_string()
    }
}

#[cfg(test)]
fn check(input: &str, expected_tree: expect_test::Expect) {
    let parse = Parser::new(input).parse();
    expected_tree.assert_eq(&parse.debug_tree());
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::expect;

    #[test]
    fn parse_nothing() {
        check("", expect![[r#"Root@0..0"#]]);
    }
}
