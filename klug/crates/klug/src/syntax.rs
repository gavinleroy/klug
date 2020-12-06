use crate::lexer::SyntaxKind;
use num_traits::{FromPrimitive, ToPrimitive};

pub(crate) type SyntaxNode = rowan::SyntaxNode<KlugLanguage>;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum KlugLanguage {}

impl rowan::Language for KlugLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        Self::Kind::from_u16(raw.0).unwrap()
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.to_u16().unwrap())
    }
}
