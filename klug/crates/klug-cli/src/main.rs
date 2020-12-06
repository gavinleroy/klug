use klug::parser::Parser;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut input = String::new();

    loop {
//        write!(stdout, "klug → ")?;
        write!(stdout, "klug \u{1F449} ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        let parse = Parser::new(&input).parse();
        println!("{}", parse.debug_tree());

        input.clear();
    }
}
