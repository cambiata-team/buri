use crate::{Target, TargetName};

#[derive(Debug, PartialEq)]
pub enum TargetParseError {
    // TODO: support relative targets
    IsNotAbsolute,
    InvalidDirectoryCharacter(char),
    InvalidTargetCharacter(char),
}

fn is_valid_part_character(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '-'
}

pub fn parse_target(input: &str) -> Result<Target, TargetParseError> {
    let mut str = input;
    if !str.starts_with("//") {
        return Err(TargetParseError::IsNotAbsolute);
    }
    str = &str[2..]; // Removes the leading "//"

    let mut directories = Vec::new();
    let mut current_directory = String::new();
    let mut target_start_index = 0;
    for c in str.chars() {
        target_start_index += 1;
        match c {
            ':' => {
                break;
            }
            '/' => {
                directories.push(current_directory);
                current_directory = String::new();
            }
            _ if is_valid_part_character(c) => {
                current_directory.push(c);
            }
            _ => {
                return Err(TargetParseError::InvalidDirectoryCharacter(c));
            }
        }
    }
    if !current_directory.is_empty() {
        directories.push(current_directory);
    }

    // validate target
    let target_name = &str[target_start_index..];

    if target_name.is_empty() {
        let last_directory = directories.last();
        if let Some(last_directory) = last_directory {
            return Ok(Target {
                name: TargetName::Specific(last_directory.to_string()),
                directories,
            });
        }
    }

    for c in target_name.chars() {
        if !is_valid_part_character(c) {
            return Err(TargetParseError::InvalidTargetCharacter(c));
        }
    }

    Ok(Target {
        name: TargetName::Specific(target_name.to_string()),
        directories,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_specific_target_names() {
        let tests = [
            ["//foo", "foo"],
            ["//foo:bar", "bar"],
            ["//foo/bar", "bar"],
            ["//foo/bar/baz", "baz"],
        ];
        for test in tests.iter() {
            let target = parse_target(test[0]).unwrap();
            assert_eq!(target.name, TargetName::Specific(test[1].to_string()));
        }
    }

    #[test]
    fn test_directories() {
        struct Test<'a> {
            input: &'a str,
            expected: Vec<&'a str>,
        }
        let tests = [
            Test {
                input: "//foo",
                expected: Vec::from(["foo"]),
            },
            Test {
                input: "//foo:bar",
                expected: Vec::from(["foo"]),
            },
            Test {
                input: "//foo/bar",
                expected: Vec::from(["foo", "bar"]),
            },
            Test {
                input: "//foo/bar/baz",
                expected: Vec::from(["foo", "bar", "baz"]),
            },
        ];

        for test in tests.iter() {
            let target = parse_target(test.input).unwrap();
            assert_eq!(target.directories, test.expected);
        }
    }
}
