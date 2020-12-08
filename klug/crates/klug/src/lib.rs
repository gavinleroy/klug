pub mod parser;
mod lexer;
mod syntax;
mod literal;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
