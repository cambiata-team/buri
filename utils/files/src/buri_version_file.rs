use std::fmt;

#[derive(Debug, PartialEq)]
pub struct BuriVersionFile {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl fmt::Display for BuriVersionFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

enum CurrentDigit {
    Major,
    Minor,
    Patch,
}

#[derive(Debug, Eq, PartialEq)]
pub enum BuriVersionFileParseError {
    IllegalCharacter(char),
    TooManySegments,
}

impl BuriVersionFile {
    pub fn from_string(version: &str) -> Result<Self, BuriVersionFileParseError> {
        let version = version.trim();
        let mut major = 0;
        let mut minor = 0;
        let mut patch = 0;
        let mut current_digit = CurrentDigit::Major;

        for c in version.chars() {
            match c {
                '0'..='9' => match current_digit {
                    CurrentDigit::Major => {
                        major *= 10;
                        major += c.to_digit(10).unwrap();
                    }
                    CurrentDigit::Minor => {
                        minor *= 10;
                        minor += c.to_digit(10).unwrap();
                    }
                    CurrentDigit::Patch => {
                        patch *= 10;
                        patch += c.to_digit(10).unwrap();
                    }
                },
                '.' => match current_digit {
                    CurrentDigit::Major => {
                        current_digit = CurrentDigit::Minor;
                    }
                    CurrentDigit::Minor => {
                        current_digit = CurrentDigit::Patch;
                    }
                    CurrentDigit::Patch => {
                        return Err(BuriVersionFileParseError::TooManySegments);
                    }
                },
                _ => {
                    return Err(BuriVersionFileParseError::IllegalCharacter(c));
                }
            }
        }
        Ok(Self {
            major,
            minor,
            patch,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serializes_file_with_semantic_version_1_2_3() {
        let file = BuriVersionFile {
            major: 1,
            minor: 2,
            patch: 3,
        };
        assert_eq!(file.to_string(), "1.2.3");
    }

    #[test]
    fn serializes_file_with_semantic_version_4_5_2() {
        let file = BuriVersionFile {
            major: 4,
            minor: 5,
            patch: 2,
        };
        assert_eq!(file.to_string(), "4.5.2");
    }

    #[test]
    fn parses_file_with_semantic_version_1_2_3() {
        let file = BuriVersionFile::from_string("1.2.3").unwrap();
        assert_eq!(
            file,
            BuriVersionFile {
                major: 1,
                minor: 2,
                patch: 3,
            }
        );
    }

    #[test]
    fn parses_file_with_semantic_version_4_5_2() {
        let file = BuriVersionFile::from_string("4.5.2").unwrap();
        assert_eq!(
            file,
            BuriVersionFile {
                major: 4,
                minor: 5,
                patch: 2,
            }
        );
    }

    #[test]
    fn trim_whitespace() {
        let file = BuriVersionFile::from_string("  1.2.3  ").unwrap();
        assert_eq!(
            file,
            BuriVersionFile {
                major: 1,
                minor: 2,
                patch: 3,
            }
        );
    }

    #[test]
    fn parse_error_on_illegal_character() {
        let file = BuriVersionFile::from_string("1.2.3a");
        assert_eq!(file, Err(BuriVersionFileParseError::IllegalCharacter('a')));
    }

    #[test]
    fn parse_error_on_too_many_segments() {
        let file = BuriVersionFile::from_string("1.1.1.1");
        assert_eq!(file, Err(BuriVersionFileParseError::TooManySegments));
    }
}
