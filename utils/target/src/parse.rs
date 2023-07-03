use crate::{Index, Target, TargetName};

#[derive(Debug, PartialEq)]
pub enum TargetParseError {
    InvalidDirectoryCharacter,
    InvalidTargetCharacter,
    TargetMissing,
    TooShort,
}

fn is_valid_part_character(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '-'
}

// TODO: convert this into a standard state machine to simplify the code.
// TODO: check that every directory has at least one character in it's name
pub fn parse_target(str: &str) -> Result<Target, TargetParseError> {
    if str.is_empty() {
        return Err(TargetParseError::TooShort);
    }
    let mut iterator = str.char_indices();
    let mut final_directory_start = 0;
    let mut index = 2;
    for (current_index, char) in iterator.by_ref() {
        index = current_index;
        match char {
            ':' => {
                break;
            }
            '/' => {
                final_directory_start = current_index + 1;
            }
            _ if is_valid_part_character(char) => {}
            _ => {
                return Err(TargetParseError::InvalidDirectoryCharacter);
            }
        }
    }

    let mut directories_end = index;
    let mut target_start_index = index + 1;
    // validate target
    if target_start_index == str.len() {
        if final_directory_start != directories_end {
            target_start_index = final_directory_start;
            directories_end += 1;
        } else {
            return Err(TargetParseError::TargetMissing);
        }
    }

    for (_, char) in iterator {
        if !is_valid_part_character(char) {
            return Err(TargetParseError::InvalidTargetCharacter);
        }
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
        // TODO: uncomment these test cases
        let tests = [
            "",
            // ":",
            // "\\",
            "hello world",
            "foo/bar...",
            "foo/bar:baz...",
            "foo/bar:baz:...",
            "foo/.../bar",
            "foo ",
            // "/hello",
            // "hello/",
            "foo/bar:baz/qux",
            // "//hello",
            "...:foo",
        ];
        for test in tests.iter() {
            let result = parse_target(test);
            println!("{:?}", result);
            assert!(result.is_err());
        }
    }
}
