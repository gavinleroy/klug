use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    let mut input = String::new();
    let mut env = klug::env::Env::default();

    loop {
        write!(stdout, "klug> ");
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        match run(input.trim(), &mut env) {
            Ok(Some(val)) => writeln!(stdout, "{}", val)?,
            Ok(None) => { }
            Err(msg) => writeln!(stderr, "{}", msg)?,
        }
        input.clear();
    }
}

fn run(s: &str, env: &mut klug::Env) -> Result<Option<klug::Value>, String> {
    let parse = klug::parse(s)
        .map_err(|msg| format!("ERR: {}", msg))?;
    let ev = parse
        .eval(env)
        .map_err(|msg| format!("ERR: {}", msg))?;

    if ev == klug::Value::Unit {
        Ok(None)
    } else {
        Ok(Some(ev))
    }
}
