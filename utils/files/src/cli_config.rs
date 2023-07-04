use std::fmt;

use serde::{Deserialize, Serialize};
use version::{is_valid_version, normalize_version};

// Do not change. This will lead to incompatibilities between versions.
pub const CLI_CONFIG_FILE_NAME: &str = ".burirc.toml";
// Do not change. This will lead to incompatibilities between versions.
pub const BURI_VERSION_KEY: &str = "buri_version";

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CliConfig {
    // Do not change. This will lead to incompatibilities between versions.
    buri_version: Option<String>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum CliConfigParseError {
    InvalidVersion,
    DeserializationError,
}

#[derive(Debug, Eq, PartialEq)]
pub enum SetVersionError {
    InvalidVersion,
}

impl fmt::Display for CliConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", toml::to_string(self).unwrap())
    }
}

impl CliConfig {
    pub fn new() -> Self {
        Self { buri_version: None }
    }

    pub fn from(contents: &str) -> Result<Self, CliConfigParseError> {
        let mut file = toml::from_str::<CliConfig>(contents)
            .map_err(|_| CliConfigParseError::DeserializationError)?;
        if let Some(version) = &file.buri_version {
            if !is_valid_version(version) {
                return Err(CliConfigParseError::InvalidVersion);
            }
            file.buri_version = Some(normalize_version(version).to_string());
        }
        Ok(file)
    }

    pub fn get_version(&self) -> Option<String> {
        self.buri_version.clone()
    }

    pub fn set_version(&mut self, version: &str) -> Result<(), SetVersionError> {
        if !is_valid_version(version) {
            return Err(SetVersionError::InvalidVersion);
        }
        self.buri_version = Some(normalize_version(version).to_string());
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_semantic_version() {
        let file = CliConfig::from("buri_version=\"1.2.3\"").unwrap();
        assert_eq!(file.get_version(), Some("1.2.3".to_string()));
    }

    #[test]
    fn parses_date_version() {
        let file = CliConfig::from("buri_version=\"2023-07-03\"").unwrap();
        assert_eq!(file.get_version(), Some("2023-07-03".to_string()));
    }

    #[test]
    fn parses_file_with_semantic_version_4_5_2() {
        let file = CliConfig::from("buri_version=\"4.5.2\"").unwrap();
        assert_eq!(file.get_version(), Some("4.5.2".to_string()));
    }

    #[test]
    fn errors_with_whitespace_around_version() {
        let result = CliConfig::from("buri_version=\"  1.2.3  \"");
        assert_eq!(result, Err(CliConfigParseError::InvalidVersion));
    }

    #[test]
    fn normalizes_version() {
        let file = CliConfig::from("buri_version=\"v1.2.3\"").unwrap();
        assert_eq!(file.get_version(), Some("1.2.3".to_string()));
    }

    #[test]
    fn parse_error_on_illegal_character() {
        let result = CliConfig::from("1.2 3");
        assert!(result.is_err());
    }

    #[test]
    fn parse_error_on_illegal_version_characters() {
        let result = CliConfig::from("buri_version=\"1.2 3\"");
        assert_eq!(result, Err(CliConfigParseError::InvalidVersion));
    }
}
