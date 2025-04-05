/// Converts the first character of a string to uppercase, leaving the rest unchanged.
///
/// # Examples
///
/// ```
/// let result = title_case("hello");
/// assert_eq!(result, "Hello");
///
/// let empty = title_case("");
/// assert_eq!(empty, "");
/// ```
pub fn title_case(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first_char) => first_char.to_uppercase().collect::<String>() + chars.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title_case() {
        assert_eq!(title_case("hello"), "Hello");
        assert_eq!(title_case(""), "");
        assert_eq!(title_case("a"), "A");
        assert_eq!(title_case("already Capitalized"), "Already Capitalized");
    }
}