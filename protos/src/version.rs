use url::Url;

include!(concat!(env!("OUT_DIR"), "/version.rs"));

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VersionInfoError {
    UnspecifiedProgram,
    UnspecifiedArchitecture,
    UnspecifiedOperatingSystemFamily,
    UnspecifiedVersionNumber,
    InvalidVersion(SemanticVersionError),
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
/// Note: this function does not ensure that the URL is reachable or secure.
/// It just uses some heuristics to check for obviously insecure URLs.
pub fn validate_download_url(url: &str) -> Result<(), DownloadUrlError> {
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

/// Validates the version info message. Note: this does not guarantee the version exists.
/// It just checks that the message is well-formed.
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
            validate_semantic_version(version_number).map_err(VersionInfoError::InvalidVersion)?;
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SemanticVersionError {
    UnspecifiedMajorVersion,
    UnspecifiedMinorVersion,
    UnspecifiedPatchVersion,
}

pub fn validate_semantic_version(
    semantic_version: &SemanticVersion,
) -> Result<(), SemanticVersionError> {
    if semantic_version.major.is_none() {
        return Err(SemanticVersionError::UnspecifiedMajorVersion);
    }
    if semantic_version.minor.is_none() {
        return Err(SemanticVersionError::UnspecifiedMinorVersion);
    }
    if semantic_version.patch.is_none() {
        return Err(SemanticVersionError::UnspecifiedPatchVersion);
    }
    Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GetVersionDownloadInfoError {
    ProgramNotSpecified,
    VersionNotSpecified,
    InvalidSemanticVersion(SemanticVersionError),
    ArchitectureNotSpecified,
    OperatingSystemFamilyNotSpecified,
    SupportedHashFunctionsNotSpecified,
    InvalidSupportedHashFunction,
}

pub fn validate_get_version_download_info_request(
    request: &GetVersionDownloadInfoRequest,
) -> Result<(), GetVersionDownloadInfoError> {
    if request.program() == Program::Unspecified {
        return Err(GetVersionDownloadInfoError::ProgramNotSpecified);
    }
    match &request.version {
        Some(get_version_download_info_request::Version::Channel(channel)) => {
            if matches!(
                Channel::from_i32(*channel),
                Some(Channel::Unspecified) | None
            ) {
                return Err(GetVersionDownloadInfoError::VersionNotSpecified);
            }
        }
        Some(get_version_download_info_request::Version::VersionNumber(semantic_version)) => {
            validate_semantic_version(semantic_version)
                .map_err(GetVersionDownloadInfoError::InvalidSemanticVersion)?;
        }
        None => {
            return Err(GetVersionDownloadInfoError::VersionNotSpecified);
        }
    }
    if request.architecture() == Architecture::Unspecified {
        return Err(GetVersionDownloadInfoError::ArchitectureNotSpecified);
    }
    if request.operating_system_family() == OperatingSystemFamily::Unspecified {
        return Err(GetVersionDownloadInfoError::OperatingSystemFamilyNotSpecified);
    }
    if request.supported_hash_functions.is_empty() {
        return Err(GetVersionDownloadInfoError::SupportedHashFunctionsNotSpecified);
    }
    for hash_function in &request.supported_hash_functions {
        if matches!(
            HashFunction::from_i32(*hash_function),
            Some(HashFunction::Unspecified) | None
        ) {
            return Err(GetVersionDownloadInfoError::InvalidSupportedHashFunction);
        }
    }
    Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParseSemanticVersionError {
    MissingMajorVersion,
    MissingMinorVersion,
    MissingPatchVersion,
    InvalidMajorVersion(std::num::ParseIntError),
    InvalidMinorVersion(std::num::ParseIntError),
    InvalidPatchVersion(std::num::ParseIntError),
}

// Input can be either format: "v.1.2.3" or "1.2.3"
pub fn parse_semantic_version_from_string(
    input: &str,
) -> Result<SemanticVersion, ParseSemanticVersionError> {
    let mut semantic_version = SemanticVersion::default();
    let mut input = input;
    if input.starts_with('v') {
        input = &input[1..];
    }
    let mut split = input.split('.');
    semantic_version.major = Some(
        split
            .next()
            .ok_or(ParseSemanticVersionError::MissingMajorVersion)?
            .parse()
            .map_err(ParseSemanticVersionError::InvalidMajorVersion)?,
    );
    semantic_version.minor = Some(
        split
            .next()
            .ok_or(ParseSemanticVersionError::MissingMinorVersion)?
            .parse()
            .map_err(ParseSemanticVersionError::InvalidMinorVersion)?,
    );
    semantic_version.patch = Some(
        split
            .next()
            .ok_or(ParseSemanticVersionError::MissingPatchVersion)?
            .parse()
            .map_err(ParseSemanticVersionError::InvalidPatchVersion)?,
    );
    Ok(semantic_version)
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
            Err(VersionInfoError::InvalidVersion(
                SemanticVersionError::UnspecifiedMajorVersion
            ))
        );
    }

    #[test]
    fn version_info_must_include_minor_version() {
        let mut version_info = make_valid_version_info();
        version_info.version_number.as_mut().unwrap().minor = None;
        assert_eq!(
            validate_version_info_message(&version_info),
            Err(VersionInfoError::InvalidVersion(
                SemanticVersionError::UnspecifiedMinorVersion
            ))
        );
    }

    #[test]
    fn version_info_must_include_patch_version() {
        let mut version_info = make_valid_version_info();
        version_info.version_number.as_mut().unwrap().patch = None;
        assert_eq!(
            validate_version_info_message(&version_info),
            Err(VersionInfoError::InvalidVersion(
                SemanticVersionError::UnspecifiedPatchVersion
            ))
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
    fn github_download_url_is_valid() {
        assert_eq!(
            // An actual download URL from an uploaded asset on GitHub.
            validate_download_url("https://github.com/cambiata-team/buri/releases/download/0.2.0/cli-aarch64-apple-darwin.tar.gz"),
            Ok(())
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

    fn create_valid_get_version_download_info_request() -> GetVersionDownloadInfoRequest {
        let mut request = GetVersionDownloadInfoRequest::default();
        request.set_program(Program::VersionManager);
        request.version = Some(get_version_download_info_request::Version::Channel(
            Channel::Latest.into(),
        ));
        request.set_architecture(Architecture::Arm64);
        request.set_operating_system_family(OperatingSystemFamily::Linux);
        request
            .supported_hash_functions
            .push(HashFunction::Sha256.into());
        request
    }

    #[test]
    fn valid_get_version_download_info_request_succeeds() {
        assert_eq!(
            validate_get_version_download_info_request(
                &create_valid_get_version_download_info_request()
            ),
            Ok(())
        );
    }

    #[test]
    fn get_version_download_info_request_must_include_program() {
        let mut request = create_valid_get_version_download_info_request();
        request.set_program(Program::Unspecified);
        assert_eq!(
            validate_get_version_download_info_request(&request),
            Err(GetVersionDownloadInfoError::ProgramNotSpecified)
        );
    }

    #[test]
    fn get_version_download_info_request_must_include_version() {
        let mut request = create_valid_get_version_download_info_request();
        request.version = None;
        assert_eq!(
            validate_get_version_download_info_request(&request),
            Err(GetVersionDownloadInfoError::VersionNotSpecified)
        );
    }

    #[test]
    fn get_version_download_info_request_must_include_architecture() {
        let mut request = create_valid_get_version_download_info_request();
        request.set_architecture(Architecture::Unspecified);
        assert_eq!(
            validate_get_version_download_info_request(&request),
            Err(GetVersionDownloadInfoError::ArchitectureNotSpecified)
        );
    }

    #[test]
    fn get_version_download_info_request_must_include_operating_system_family() {
        let mut request = create_valid_get_version_download_info_request();
        request.set_operating_system_family(OperatingSystemFamily::Unspecified);
        assert_eq!(
            validate_get_version_download_info_request(&request),
            Err(GetVersionDownloadInfoError::OperatingSystemFamilyNotSpecified)
        );
    }

    #[test]
    fn get_version_download_info_request_must_include_supported_hash_functions() {
        let mut request = create_valid_get_version_download_info_request();
        request.supported_hash_functions.clear();
        assert_eq!(
            validate_get_version_download_info_request(&request),
            Err(GetVersionDownloadInfoError::SupportedHashFunctionsNotSpecified)
        );
    }

    #[test]
    fn get_version_download_info_request_must_include_supported_hash_function() {
        let mut request = create_valid_get_version_download_info_request();
        request.supported_hash_functions[0] = HashFunction::Unspecified.into();
        assert_eq!(
            validate_get_version_download_info_request(&request),
            Err(GetVersionDownloadInfoError::InvalidSupportedHashFunction)
        );
    }

    #[test]
    fn parse_semantic_version_from_string_parses_major_version() {
        assert_eq!(
            parse_semantic_version_from_string("1.0.42").unwrap().major,
            Some(1)
        );
    }

    #[test]
    fn parse_semantic_version_from_string_parses_minor_version() {
        assert_eq!(
            parse_semantic_version_from_string("1.0.42").unwrap().minor,
            Some(0)
        );
    }

    #[test]
    fn parse_semantic_version_from_string_parses_patch_version() {
        assert_eq!(
            parse_semantic_version_from_string("1.0.42").unwrap().patch,
            Some(42)
        );
    }

    #[test]
    fn parse_semantic_version_from_string_parses_major_version_with_preceding_v() {
        assert_eq!(
            parse_semantic_version_from_string("v1.0.42").unwrap().major,
            Some(1)
        );
    }

    #[test]
    fn parse_semantic_version_from_string_parses_minor_version_with_preceding_v() {
        assert_eq!(
            parse_semantic_version_from_string("v1.0.42").unwrap().minor,
            Some(0)
        );
    }

    #[test]
    fn parse_semantic_version_from_string_parses_patch_version_with_preceding_v() {
        assert_eq!(
            parse_semantic_version_from_string("v1.0.42").unwrap().patch,
            Some(42)
        );
    }

    #[test]
    fn parse_semantic_version_from_string_errors_with_invalid_input() {
        let tests = ["1.0", "1", "v1", "v1.0", "hello world", "    v1.2.3"];
        for test in tests.iter() {
            assert!(parse_semantic_version_from_string(test).is_err());
        }
    }
}
