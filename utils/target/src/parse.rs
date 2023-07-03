use crate::{Index, Target, TargetName};

#[derive(Debug, PartialEq)]
pub enum TargetParseError {
    TooShort,
    IllegalCharacter,
    MissingTargetName,
    DirectoriesMustHaveAName,
    CannotStartWithASlash,
}

fn is_valid_part_character(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '-'
}

pub fn parse_target(str: &str) -> Result<Target, TargetParseError> {
    if str.is_empty() {
        return Err(TargetParseError::TooShort);
    }
    if str.ends_with(':') || str.ends_with('/') {
        return Err(TargetParseError::MissingTargetName);
    }
    let mut target_start_index = 0;
    let mut directories_end = str.len();
    let mut has_set_target_start = false;
    let mut has_seen_slash = false;
    let mut is_previous_char_slash = false;
    // Go in reverse in case the target name is implicitly the directory.
    for (iterator_index, char) in str.chars().rev().enumerate() {
        let index = str.len() - iterator_index - 1;
        match char {
            c if is_valid_part_character(c) => {}
            ':' => {
                if has_seen_slash || has_set_target_start {
                    return Err(TargetParseError::IllegalCharacter);
                }
                target_start_index = index + 1;
                has_set_target_start = true;
                directories_end = index;
            }
            '/' => {
                if is_previous_char_slash {
                    return Err(TargetParseError::DirectoriesMustHaveAName);
                }
                if index == 0 {
                    return Err(TargetParseError::CannotStartWithASlash);
                }
                if !has_set_target_start {
                    target_start_index = index + 1;
                    has_set_target_start = true;
                }
                has_seen_slash = true;
            }
            _ => return Err(TargetParseError::IllegalCharacter),
        }
        is_previous_char_slash = char == '/';
    }

    Ok(Target {
        name: TargetName::Specific(target_start_index as Index),
        directories_end: directories_end as Index,
        raw_text: str.to_string(),
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_specific_target_names() {
        let tests = [
            ["foo", "foo"],
            ["foo:bar", "bar"],
            ["foo/bar", "bar"],
            ["foo/bar/foobar", "foobar"],
        ];
        for test in tests.iter() {
            let target = parse_target(test[0]).unwrap();
            assert_eq!(target.name(), test[1].to_string());
        }
    }

    #[test]
    fn test_directories() {
        let tests = [
            ["foo", "foo"],
            ["foo:bar", "foo"],
            ["foo/bar", "foo/bar"],
            ["foo/bar/baz", "foo/bar/baz"],
        ];
        for test in tests.iter() {
            let target = parse_target(test[0]).unwrap();
            assert_eq!(target.get_directories(), test[1]);
        }
    }

    #[test]
    fn errors_on_invalid_targets() {
        let tests = [
            "",
            ":",
            "\\",
            "hello world",
            "foo/bar...",
            "foo/bar:baz...",
            "foo/bar:baz:...",
            "foo/.../bar",
            "foo ",
            "/hello",
            "hello/",
            "foo/bar:baz/qux",
            "//hello",
            "...:foo",
            "foo:bar:baz",
            "foo:bar/baz",
            "foo::bar",
        ];
        for test in tests.iter() {
            let result = parse_target(test);
            println!("{:?}", result);
            assert!(result.is_err());
        }
    }
}
