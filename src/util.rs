pub(crate) fn to_camel_case(s: &str, separator: Option<char>) -> String {
    let sep = separator.unwrap_or('-');

    s.split(sep)
        .enumerate()
        .map(|(i, part)| {
            if i == 0 {
                part.to_string()
            } else {
                let mut chars = part.chars();
                if let Some(first) = chars.next() {
                    format!("{}{}", first.to_ascii_uppercase(), chars.as_str())
                } else {
                    String::new()
                }
            }
        })
        .collect()
}
