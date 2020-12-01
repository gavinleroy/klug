const WHITESPACE: &[char] = &[' ', '\n'];

pub(crate) fn tag<'a, 'b>(t: &'a str, s: &'b str) -> Result<&'b str, String> {
    if s.starts_with(t) {
        Ok(&s[t.len()..])
    } else {
        Err(format!("expected {}", t))
    }
}

pub(crate) fn extract_ident(s: &str) -> Result<(&str, &str), String> {
    let okay = s
        .chars()
        .next()
        .map(|c| c.is_ascii_alphabetic())
        .unwrap_or(false);

    if okay {
        Ok(take_while(|c| c.is_ascii_alphanumeric(), s))
    } else {
        Err("expected identifier".to_string())
    }
}

fn take_while(acc: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    let end = s 
        .char_indices() 
        .find_map(|(idx, c)| if acc(c) { None } else { Some(idx) }) 
        .unwrap_or_else(|| s.len());

    (&s[..end], &s[end..])
}

fn take_while1(
    acc: impl Fn(char) -> bool, 
    s: &str,
    error_msg: String) -> Result<(&str, &str), String> {
    let (extracted, remainder) = take_while(acc, s);

    if extracted.is_empty() {
        Err(error_msg)
    } else {
        Ok((extracted, remainder))
    }
}

pub(crate) fn extract_digits(s: &str) -> Result<(&str, &str), String> {
    take_while1(|c| c.is_ascii_digit(), s, "expected digit".to_string())
}

pub(crate) fn extract_whitespace(s: &str) -> (&str, &str) {
    take_while(|c| WHITESPACE.contains(&c), s)
}

pub(crate) fn extract_whitespace1(s: &str) -> Result<(&str, &str), String> {
    take_while1(|c| WHITESPACE.contains(&c), s, "expected a space".to_string())
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
    fn extract_one_digit() {
        assert_eq!(extract_digits("1+2"), Ok(("1", "+2")));
    }
    #[test]
    fn extract_multiple_digits() {
        assert_eq!(extract_digits("10-20"), Ok(("10", "-20")));
    }
    #[test]
    fn extract_digits_with_no_remainder() {
        assert_eq!(extract_digits("100"), Ok(("100", "")));
    }
    #[test]
    fn do_not_extract_digits_when_input_is_invalid() {
        assert_eq!(extract_digits("abcd"), Err("expected digit".to_string()));
    }
    #[test]
    fn cannot_extract_ident_beginning_with_number() {
        assert_eq!(
            extract_ident("123abc"),
            Err("expected identifier".to_string()),
        );
    }
}
