use std::cmp::Ordering;

pub fn trim_start<'a>(input: &'a str, start: &str) -> Option<&'a str> {
    if input.starts_with(start) {
        Some(&input[start.bytes().len()..])
    } else {
        None
    }
}

#[allow(dead_code)]
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

pub fn which_comes_first<'a>(input: &str, a: &'a str, b: &'a str) -> Option<(&'a str, usize)> {
    let len_a = input.find(a);
    let len_b = input.find(b);

    match (len_a, len_b) {
        (None, None) => None,
        (None, Some(len)) => Some((b, len)),
        (Some(len), None) => Some((a, len)),
        (Some(len_a), Some(len_b)) => match len_a.cmp(&len_b) {
            Ordering::Less => Some((b, len_b)),
            Ordering::Equal => {
                if a.bytes().len() > b.bytes().len() {
                    Some((a, len_a))
                } else {
                    Some((b, len_b))
                }
            }
            Ordering::Greater => Some((a, len_a)),
        },
    }
}
