use url::Url;

include!(concat!(env!("OUT_DIR"), "/version.rs"));

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VersionInfoError {
    UnspecifiedProgram,
    UnspecifiedArchitecture,
    UnspecifiedOperatingSystemFamily,
    UnspecifiedVersionNumber,
    UnspecifiedMajorVersion,
    UnspecifiedMinorVersion,
    UnspecifiedPatchVersion,
    MissingChecksums,
    UnspecifiedHashFunction,
    EmptyChecksumHash,
    MissingDownloadUrls,
    InvalidDownloadUrl(DownloadUrlError),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DownloadUrlError {
    EmptyUrl,
    InvalidUrl,
    InsecureUrl,
    IncludesUsername,
    IncludesPassword,
    IncludesPort,
    UnknownHost,
}

/// Validates the download URL and checks for several potential security issues.
fn validate_download_url(url: &str) -> Result<(), DownloadUrlError> {
    if url.is_empty() {
        return Err(DownloadUrlError::EmptyUrl);
    }
    let parsed_url = Url::parse(url).map_err(|_| DownloadUrlError::InvalidUrl)?;
    if parsed_url.scheme() != "https" {
        return Err(DownloadUrlError::InsecureUrl);
    }
    if !parsed_url.username().is_empty() {
        return Err(DownloadUrlError::IncludesUsername);
    }
    if parsed_url.password().is_some() {
        return Err(DownloadUrlError::IncludesPassword);
    }
    if parsed_url.port().is_some() {
        return Err(DownloadUrlError::IncludesPort);
    }
    if !matches!(
        parsed_url.host_str(),
        Some("github.com") | Some("downloads.buri-lang.dev"),
    ) {
        return Err(DownloadUrlError::UnknownHost);
    }
    Ok(())
}

pub fn validate_version_info_message(version_info: &VersionInfo) -> Result<(), VersionInfoError> {
    if version_info.program() == Program::Unspecified {
        return Err(VersionInfoError::UnspecifiedProgram);
    }
    if version_info.architecture() == Architecture::Unspecified {
        return Err(VersionInfoError::UnspecifiedArchitecture);
    }
    if version_info.operating_system_family() == OperatingSystemFamily::Unspecified {
        return Err(VersionInfoError::UnspecifiedOperatingSystemFamily);
    }
    match &version_info.version_number {
        Some(version_number) => {
            if version_number.major.is_none() {
                return Err(VersionInfoError::UnspecifiedMajorVersion);
            }
            if version_number.minor.is_none() {
                return Err(VersionInfoError::UnspecifiedMinorVersion);
            }
            if version_number.patch.is_none() {
                return Err(VersionInfoError::UnspecifiedPatchVersion);
            }
        }
        None => {
            return Err(VersionInfoError::UnspecifiedVersionNumber);
        }
    }
    if version_info.checksums.is_empty() {
        return Err(VersionInfoError::MissingChecksums);
    }
    for checksum in &version_info.checksums {
        if checksum.hash_function() == HashFunction::Unspecified {
            return Err(VersionInfoError::UnspecifiedHashFunction);
        }
        if checksum.checksum.is_empty() {
            return Err(VersionInfoError::EmptyChecksumHash);
        }
    }
    if version_info.download_urls.is_empty() {
        return Err(VersionInfoError::MissingDownloadUrls);
    }
    for url in &version_info.download_urls {
        validate_download_url(url).map_err(VersionInfoError::InvalidDownloadUrl)?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    fn make_valid_version_info() -> VersionInfo {
        let mut version_info = VersionInfo::default();
        version_info.set_program(Program::VersionManager);
        version_info.set_architecture(Architecture::Arm64);
        version_info.set_operating_system_family(OperatingSystemFamily::Linux);
        version_info.version_number = Some(SemanticVersion {
            major: Some(0),
            minor: Some(1),
            patch: Some(0),
        });
        let mut checksum = Checksum::default();
        checksum.set_hash_function(HashFunction::Sha256);
        checksum.checksum = "deadbeef".to_string();
        version_info.checksums.push(checksum);
        version_info
            .download_urls
            .push("https://downloads.buri-lang.dev".to_string());

        version_info
    }

    #[test]
    fn valid_version_info_returns_ok() {
        let version_info = make_valid_version_info();
        assert_eq!(validate_version_info_message(&version_info), Ok(()));
    }

    #[test]
    fn version_info_must_include_program() {
        let mut version_info = make_valid_version_info();
        version_info.set_program(Program::Unspecified);
        assert_eq!(
            validate_version_info_message(&version_info),
            Err(VersionInfoError::UnspecifiedProgram)
        );
    }

    #[test]
    fn version_info_must_include_architecture() {
        let mut version_info = make_valid_version_info();
        version_info.set_architecture(Architecture::Unspecified);
        assert_eq!(
            validate_version_info_message(&version_info),
            Err(VersionInfoError::UnspecifiedArchitecture)
        );
    }

    #[test]
    fn version_info_must_include_operating_system_family() {
        let mut version_info = make_valid_version_info();
        version_info.set_operating_system_family(OperatingSystemFamily::Unspecified);
        assert_eq!(
            validate_version_info_message(&version_info),
            Err(VersionInfoError::UnspecifiedOperatingSystemFamily)
        );
    }

    #[test]
    fn version_info_must_include_version_number() {
        let mut version_info = make_valid_version_info();
        version_info.version_number = None;
        assert_eq!(
            validate_version_info_message(&version_info),
            Err(VersionInfoError::UnspecifiedVersionNumber)
        );
    }

    #[test]
    fn version_info_must_include_major_version() {
        let mut version_info = make_valid_version_info();
        version_info.version_number.as_mut().unwrap().major = None;
        assert_eq!(
            validate_version_info_message(&version_info),
            Err(VersionInfoError::UnspecifiedMajorVersion)
        );
    }

    #[test]
    fn version_info_must_include_minor_version() {
        let mut version_info = make_valid_version_info();
        version_info.version_number.as_mut().unwrap().minor = None;
        assert_eq!(
            validate_version_info_message(&version_info),
            Err(VersionInfoError::UnspecifiedMinorVersion)
        );
    }

    #[test]
    fn version_info_must_include_patch_version() {
        let mut version_info = make_valid_version_info();
        version_info.version_number.as_mut().unwrap().patch = None;
        assert_eq!(
            validate_version_info_message(&version_info),
            Err(VersionInfoError::UnspecifiedPatchVersion)
        );
    }

    #[test]
    fn version_info_must_include_checksums() {
        let mut version_info = make_valid_version_info();
        version_info.checksums.clear();
        assert_eq!(
            validate_version_info_message(&version_info),
            Err(VersionInfoError::MissingChecksums)
        );
    }

    #[test]
    fn version_info_must_include_checksum_hash_function() {
        let mut version_info = make_valid_version_info();
        version_info.checksums[0].set_hash_function(HashFunction::Unspecified);
        assert_eq!(
            validate_version_info_message(&version_info),
            Err(VersionInfoError::UnspecifiedHashFunction)
        );
    }

    #[test]
    fn version_info_must_include_checksum_hash() {
        let mut version_info = make_valid_version_info();
        version_info.checksums[0].checksum = "".to_string();
        assert_eq!(
            validate_version_info_message(&version_info),
            Err(VersionInfoError::EmptyChecksumHash)
        );
    }

    #[test]
    fn version_info_must_include_download_url() {
        let mut version_info = make_valid_version_info();
        version_info.download_urls.clear();
        assert_eq!(
            validate_version_info_message(&version_info),
            Err(VersionInfoError::MissingDownloadUrls)
        );
    }

    #[test]
    fn version_info_download_url_must_be_valid() {
        let mut version_info = make_valid_version_info();
        version_info.download_urls[0] = "not a url".to_string();
        assert_eq!(
            validate_version_info_message(&version_info),
            Err(VersionInfoError::InvalidDownloadUrl(
                DownloadUrlError::InvalidUrl
            ))
        );
    }

    #[test]
    fn download_url_must_be_valid() {
        assert_eq!(
            validate_download_url("not a url"),
            Err(DownloadUrlError::InvalidUrl)
        );
    }

    #[test]
    fn download_url_must_be_https() {
        assert_eq!(
            validate_download_url("http://downloads.buri-lang.dev"),
            Err(DownloadUrlError::InsecureUrl)
        );
    }

    #[test]
    fn download_url_must_be_from_a_known_host() {
        assert_eq!(
            validate_download_url("https://downloads.not-buri-lang.dev"),
            Err(DownloadUrlError::UnknownHost)
        );
    }

    #[test]
    fn github_is_a_known_host() {
        assert_eq!(validate_download_url("https://github.com"), Ok(()));
    }

    #[test]
    fn download_url_cannot_have_username() {
        assert_eq!(
            validate_download_url("https://username@downloads.buri-lang.dev"),
            Err(DownloadUrlError::IncludesUsername)
        );
    }

    #[test]
    fn download_url_cannot_have_password() {
        assert_eq!(
            validate_download_url("https://:password@downloads.buri-lang.dev"),
            Err(DownloadUrlError::IncludesPassword)
        );
    }

    #[test]
    fn download_url_cannot_have_port() {
        assert_eq!(
            validate_download_url("https://downloads.buri-lang.dev:8080"),
            Err(DownloadUrlError::IncludesPort)
        );
    }
}
