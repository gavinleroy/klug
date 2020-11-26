pub(crate) fn tag<'a, 'b>(t: &'a str, s: &'b str) -> &'b str {
    if s.starts_with(t) {
        &s[t.len()..]
    } else {
        panic!("expected {}", t)
    }
}

pub(crate) fn extract_ident(s: &str) -> (&str, &str) { 
    let okay = s.chars()
        .next()
        .map(|c| c.is_ascii_alphabetic())
        .unwrap_or(false);
    if okay {
        take_while(|c| c.is_ascii_alphanumeric(), s)
    } else {
        ("", s)
    }
}

pub(crate) fn take_while(acc: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    let end = s 
        .char_indices() 
        .find_map(|(idx, c)| if acc(c) { None } else { Some(idx) }) 
        .unwrap_or_else(|| s.len());

    (&s[..end], &s[end..])
}

pub(crate) fn extract_digits(s: &str) -> (&str, &str) {
    take_while(|c| c.is_ascii_digit(), s)
}

pub(crate) fn extract_whitespace(s: &str) -> (&str, &str) {
    take_while(|c| c == ' ', s)
}

pub(crate) fn extract_op(s: &str) -> (&str, &str) {
    match &s[0..1] {
        "+" | "*" | "-" | "/" => {},
        _ => panic!("no operator"),
    }
    (&s[0..1], &s[1..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn do_not_extract_anything_from_empty_input() {
        assert_eq!(extract_digits(""), ("", ""));
    }
    #[test]
    fn extract_digits_with_no_remainder() {
        assert_eq!(extract_digits("100"), ("100", ""));
    }
    #[test]
    fn extract_plus() {
        assert_eq!(extract_op("+2"), ( "+", "2"));
    }
    #[test]
    fn extract_minus() {
        assert_eq!(extract_op("-10"), ("-", "10"));
    }
    #[test]
    fn extract_star() {
        assert_eq!(extract_op("*3"), ("*", "3"));
    }
    #[test]
    fn extract_slash() {
        assert_eq!(extract_op("/4"), ("/", "4"));
    }
    #[test]
    fn extract_alphabetic_ident() {
        assert_eq!(extract_ident("abcdEFG stop"), ("abcdEFG", " stop"));
    }
    #[test]
    fn extract_alphanumeric_ident() {
        assert_eq!(extract_ident("foobar1()"), ("foobar1", "()"));
    }
    #[test]
    fn cannot_extract_ident_beginning_with_number() {
        assert_eq!(extract_ident("123abc"), ("", "123abc"));
    }
}
