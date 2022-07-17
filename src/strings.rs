pub fn trim_start<'a>(input: &'a str, start: &str) -> Option<&'a str> {
    if input.starts_with(start) {
        Some(&input[start.bytes().len()..])
    } else {
        None
    }
}

pub fn end_diff<'a>(input: &'a str, end: &str) -> Option<&'a str> {
    if input.ends_with(end) {
        let diff = input.bytes().len() - end.bytes().len();
        Some(&input[..diff])
    } else {
        None
    }
}

pub fn trim_first_line<'a>(input: &'a str) -> Option<&'a str> {
    let rest = trim_start(input, input.lines().next()?)?;
    let rest = trim_start(rest, "\n")?;

    Some(rest)
}