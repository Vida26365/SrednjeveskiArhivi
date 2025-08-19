pub fn capitalize(str: &str) -> String {
    let mut chars = str.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("hello"), "Hello");
        assert_eq!(capitalize("WoRlD"), "WoRlD");
        assert_eq!(capitalize("a"), "A");
        assert_eq!(capitalize("ab"), "Ab");
        assert_eq!(capitalize("123"), "123");
        assert_eq!(capitalize(""), "");
    }
}
