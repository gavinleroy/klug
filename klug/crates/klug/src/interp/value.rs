#[derive(Debug, PartialEq)]
pub(crate) enum Value {
    Number(f64),
    Str(String),
    Bool(bool),
}
