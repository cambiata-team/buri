use files::cli_config::SetVersionError;
use prost::DecodeError;
use protos::version::VersionInfoError;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
#[allow(dead_code)] // many of these are used in prod-only blocks.
pub enum CliError {
    VfsError(String),
    MustInitialize,
    InvalidThorVersion(String),
    NetworkError(String),
    /// Expected, actual
    ChecksumsDoNotMatch(String, String),
    NoSupportedChecksum,
    ChecksumNotValidHex(String),
    SetThorVersionError(SetVersionError),
    VersionInfoDecodeError(DecodeError),
    VersionInfoMessageError(VersionInfoError),
    NoDownloadUrls,
    TemporaryDirectoryCreationError(String),
    CreateTarballFileError(String),
    WriteToTarballFileError(String),
    UnpackTarballError(String),
    RenameThorBinaryError(String),
    CannotReadThorBinaryPermissions(String),
    CannotMarkBinaryAsExecutable(String),
    CannotRemoveTemporaryFiles(String),
    WriteToExistingConfigFileError(String),
    WriteToNewConfigFileError(String),
}

const MUST_INITIALIZE_MESSAGE: &str = "
You must be in a workspace to use Buri.
Use `buri init` to create a new workspace.

$   buri init

Use `buri --help` for more information.";

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            match self {
                Self::VfsError(message) => format!("File system error: {message}"),
                Self::MustInitialize => MUST_INITIALIZE_MESSAGE.to_string(),
                Self::InvalidThorVersion(version) => {
                    format!("Invalid Thor version: {version}")
                }
                Self::NetworkError(message) => format!("Network error: {message}"),
                Self::ChecksumsDoNotMatch(expected, actual) =>
                    format!("Checksums do not match. Expected: {expected}, actual: {actual}"),
                Self::NoSupportedChecksum => "No supported checksum".to_string(),
                Self::ChecksumNotValidHex(checksum) =>
                    format!("Checksum is not valid hex: {checksum}"),
                Self::SetThorVersionError(error) => format!("Error setting Thor version: {error}"),
                Self::VersionInfoDecodeError(error) =>
                    format!("Error decoding version info: {error}"),
                Self::VersionInfoMessageError(error) =>
                    format!("Error parsing version info: {error:?}"),
                Self::NoDownloadUrls => "No download URLs".to_string(),
                Self::TemporaryDirectoryCreationError(message) =>
                    format!("Error creating temporary directory: {message}"),
                Self::CreateTarballFileError(message) =>
                    format!("Error creating tarball file: {message}"),
                Self::WriteToTarballFileError(message) =>
                    format!("Error writing to tarball file: {message}"),
                Self::UnpackTarballError(message) => format!("Error unpacking tarball: {message}"),
                Self::RenameThorBinaryError(message) =>
                    format!("Error renaming Thor binary: {message}"),
                Self::CannotReadThorBinaryPermissions(message) =>
                    format!("Error reading Thor binary permissions: {message}"),
                Self::CannotMarkBinaryAsExecutable(message) =>
                    format!("Error marking binary as executable: {message}"),
                Self::CannotRemoveTemporaryFiles(message) =>
                    format!("Error removing temporary files: {message}"),
                Self::WriteToExistingConfigFileError(message) =>
                    format!("Error writing to existing config file: {message}"),
                Self::WriteToNewConfigFileError(message) =>
                    format!("Error writing to new config file: {message}"),
            }
        )
    }
}
