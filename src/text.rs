use colored::ColoredString;

#[derive(Clone)]
pub enum AnyString {
    CommonString(String),
    ColoredString(ColoredString),
}

impl AnyString {
    pub fn from_string(input: String) -> Self {
        Self::CommonString(input)
    }
    pub fn from_colored(input: ColoredString) -> Self {
        if input.is_plain() {
            let input_as_string: String = input.to_string();
            return Self::CommonString(input_as_string);
        }
        return Self::ColoredString(input);
    }
}

impl From<String> for AnyString {
    fn from(input: String) -> Self {
        Self::CommonString(input)
    }
}

impl From<ColoredString> for AnyString {
    fn from(input: ColoredString) -> Self {
        if input.is_plain() {
            let input_as_string: String = input.to_string();
            return Self::CommonString(input_as_string);
        }
        return Self::ColoredString(input);
    }
}

impl From<AnyString> for String {
    fn from(input: AnyString) -> Self {
        match input {
            AnyString::CommonString(valid_string) => valid_string,
            AnyString::ColoredString(invalid_string) => invalid_string.to_string(),
        }
    }
}

impl From<AnyString> for ColoredString {
    fn from(input: AnyString) -> Self {
        match input {
            AnyString::CommonString(invalid_string) => ColoredString::from(&invalid_string[..]),
            AnyString::ColoredString(valid_string) => valid_string,
        }
    }
}

impl std::fmt::Display for AnyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyString::CommonString(common_string) => write!(f, "{}", common_string),
            AnyString::ColoredString(colored_string) => write!(f, "{}", colored_string),
        }
    }
}