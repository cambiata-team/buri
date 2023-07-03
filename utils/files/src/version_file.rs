use std::fmt;
use version::{is_valid_version, normalize_version};

pub const VERSION_FILE_NAME: &str = ".buri-version";

#[derive(Debug, PartialEq)]
pub struct VersionFile<'a> {
    version: &'a str,
}

impl<'a> fmt::Display for VersionFile<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.version)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum VersionFileParseError {
    InvalidVersion,
}

impl<'a> VersionFile<'a> {
    pub fn from_string(version: &'a str) -> Result<Self, VersionFileParseError> {
        let version = version.trim();
        if is_valid_version(version) {
            return Ok(VersionFile {
                version: normalize_version(version),
            });
        }
        Err(VersionFileParseError::InvalidVersion)
    }

    pub fn get_version(&self) -> &'a str {
        self.version
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_file_with_semantic_version_1_2_3() {
        let file = VersionFile::from_string("1.2.3").unwrap();
        assert_eq!(file.get_version(), "1.2.3");
    }

    #[test]
    fn parses_file_with_semantic_version_4_5_2() {
        let file = VersionFile::from_string("4.5.2").unwrap();
        assert_eq!(file.get_version(), "4.5.2");
    }

    #[test]
    fn trim_whitespace() {
        let file = VersionFile::from_string("  1.2.3  ").unwrap();
        assert_eq!(file.get_version(), "1.2.3");
    }

    #[test]
    fn normalizes_version() {
        let file = VersionFile::from_string("v1.2.3").unwrap();
        assert_eq!(file.get_version(), "1.2.3");
    }

    #[test]
    fn parse_error_on_illegal_character() {
        let result = VersionFile::from_string("1.2 3");
        assert!(result.is_err());
    }

    #[test]
    fn parses_successfully_for_valid_versions() {
        let result = VersionFile::from_string("1.1.1.1");
        assert!(result.is_ok());
    }
}
