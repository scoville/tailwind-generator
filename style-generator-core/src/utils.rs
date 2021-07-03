macro_rules! replace_first_char {
    ($escaped_class_name:ident, $($char:literal => $replace_with:expr),*) => (
        match $escaped_class_name.chars().nth(0) {
            $(Some($char) => $escaped_class_name.replace_range(..1, $replace_with),)+
            _ => (),
        }
    )
}

pub fn escape_class_name(class: String) -> String {
    let mut escaped_class_name = class;

    replace_first_char!(escaped_class_name,
        '-' => "neg-",
        '0' => "zero-",
        '1' => "one-",
        '2' => "two-",
        '3' => "three-",
        '4' => "four-",
        '5' => "five-",
        '6' => "six-",
        '7' => "seven-",
        '8' => "eight-",
        '9' => "nine-"
    );

    escaped_class_name
        .replace(":-", "-neg-")
        .replace("/", "-over-")
        .replace(":", "-")
        .replace(".", "-dot-")
}
