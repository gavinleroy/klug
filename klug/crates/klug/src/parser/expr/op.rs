use super::*;

#[derive(Debug, PartialEq)]
pub(crate) enum InfixOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl InfixOp {
    pub(super) fn binding_power(&self) -> (u8, u8) {
        match self {
            Self::Add | Self::Sub => (1, 2),
            Self::Mul | Self::Div => (3, 4),
        }
    }

    pub(super) fn stringify(&self) -> String {
        let s = match self {
            Self::Add  => "+",
            Self::Sub => "-",
            Self::Mul  => "*",
            Self::Div => "/",
        };
        s.to_string()
    }
}


#[derive(Debug, PartialEq)]
pub(crate) enum PrefixOp {
    Neg,
    Not,
}

impl PrefixOp {
    pub(super) fn binding_power(&self) -> ((), u8) {
        match self {
            Self::Neg => ((), 5),
            Self::Not => ((), 5)
        }
    }

    pub(super) fn stringify(&self) -> String {
        let s = match self {
            Self::Neg => "-",
            Self::Not => "!",
        };
        s.to_string()
    }

    pub(super) fn from_kind(sk: SyntaxKind) -> Self {
        match sk {
            SyntaxKind::Bang => Self::Not,
            SyntaxKind::Minus => Self::Neg,
            _ => unreachable!(),
        }
    }
}

