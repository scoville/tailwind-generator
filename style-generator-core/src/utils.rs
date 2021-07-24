use std::borrow::Cow;

macro_rules! replace_first_char {
    ($escaped_class_name:ident, $($char:literal => $replace_with:expr),*) => (
        match $escaped_class_name.chars().nth(0) {
            $(Some($char) => $escaped_class_name.replace_range(..1, $replace_with),)+
            _ => (),
        }
    )
}

/// Escapes any class name special characters if needed
/// (a pointer to the provided class is returned otherwise).
///
/// Performs "smart conversion" when possible (`/` to `_over_`, etc...),
/// and returns a camel_cased string.
pub fn escape_class_name(class: &str) -> Cow<'_, str> {
    let mut class_chars = class.chars();

    // If the class name is already valid, do not perform any transformation
    if class_chars
        .next()
        .map_or_else(|| false, char::is_alphabetic)
        && class_chars.all(|c| c == '_' || (c.is_alphanumeric() && c.is_lowercase()))
    {
        return class.into();
    }

    let mut class = class.to_string();

    replace_first_char!(class,
        '-' => "neg_",
        '0' => "zero_",
        '1' => "one_",
        '2' => "two_",
        '3' => "three_",
        '4' => "four_",
        '5' => "five_",
        '6' => "six_",
        '7' => "seven_",
        '8' => "eight_",
        '9' => "nine_"
    );

    class
        .replace(":-", "_neg_")
        .replace("/", "_over_")
        .replace(".", "_dot_")
        .replace(
            &['!', ':', '~', '@', '#', '$', '^', '=', '*', '(', ')', ';'][..],
            "_",
        )
        .to_lowercase()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_cases() {
        assert_eq!(escape_class_name("foo"), "foo");
        assert_eq!(escape_class_name("foo_bar"), "foo_bar");
        assert_eq!(escape_class_name("f42"), "f42");
    }

    #[test]
    fn test_first_letter_is_alpha() {
        assert_eq!(escape_class_name("foo"), "foo");
        assert_eq!(escape_class_name("1foo_bar"), "one_foo_bar");
        assert_eq!(escape_class_name("-foo"), "neg_foo");
    }

    #[test]
    fn test_smart_conversion() {
        assert_eq!(escape_class_name("foo:-bar"), "foo_neg_bar");
        assert_eq!(escape_class_name("foo/bar"), "foo_over_bar");
    }

    #[test]
    fn test_lower_case() {
        assert_eq!(escape_class_name("foo"), "foo");
        assert_eq!(escape_class_name("fOO"), "foo");
        assert_eq!(escape_class_name("fOO_bAR"), "foo_bar");
    }

    #[test]
    fn test_escape_special_characters() {
        assert_eq!(escape_class_name("foo#"), "foo_");
        assert_eq!(escape_class_name("foo@"), "foo_");
        assert_eq!(escape_class_name("foo!"), "foo_");
    }
}
