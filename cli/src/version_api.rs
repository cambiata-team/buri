use protos::{
    encode_message_to_base_64,
    version::{
        get_version_download_info_request::Version, Architecture, Channel,
        GetVersionDownloadInfoRequest, HashFunction, OperatingSystemFamily, Program,
    },
};

fn get_architecture(str: &'static str) -> Architecture {
    match str {
        "x86_64" => Architecture::X8664,
        "aarch64" => Architecture::Arm64,
        _ => Architecture::Unspecified,
    }
}

fn get_operating_system_family(str: &'static str) -> OperatingSystemFamily {
    match str {
        "linux" => OperatingSystemFamily::Linux,
        "macos" => OperatingSystemFamily::Darwin,
        _ => OperatingSystemFamily::Unspecified,
    }
}

fn build_version_api_request_url(
    version: Version,
    architecture: &'static str,
    operating_system_family: &'static str,
) -> String {
    let mut params = GetVersionDownloadInfoRequest::default();
    params.set_program(Program::Thor);
    params.version = Some(version);
    params.set_architecture(get_architecture(architecture));
    params.set_operating_system_family(get_operating_system_family(operating_system_family));
    // This version of the CLI only supports SHA256.
    // If newer versions of the CLI support more hash functions,
    // then this should be updated. But it should always be hardcoded.
    params
        .supported_hash_functions
        .push(HashFunction::Sha256.into());

    format!(
        "https://version-api.buri-lang.dev/get-version-download-info?q={}",
        encode_message_to_base_64(&params)
    )
}

#[allow(dead_code)] // Called in impure code
pub fn build_version_api_request_url_for_latest(
    architecture: &'static str,
    operating_system_family: &'static str,
) -> String {
    build_version_api_request_url(
        Version::Channel(Channel::Latest.into()),
        architecture,
        operating_system_family,
    )
}

#[allow(dead_code)] // Called in impure code
pub fn build_version_api_request_url_for_version(
    version: &str,
    architecture: &'static str,
    operating_system_family: &'static str,
) -> String {
    build_version_api_request_url(
        Version::VersionNumber(version.to_string()),
        architecture,
        operating_system_family,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_version_api_request_url_for_latest_reaches_correct_endpoint() {
        let url = build_version_api_request_url_for_latest("x86_64", "linux");
        assert!(url.starts_with("https://version-api.buri-lang.dev/get-version-download-info?q="));
    }

    #[test]
    fn build_version_api_request_url_for_version_reaches_correct_endpoint() {
        let url = build_version_api_request_url_for_version("0.1.0", "x86_64", "linux");
        assert!(url.starts_with("https://version-api.buri-lang.dev/get-version-download-info?q="));
    }

    #[test]
    fn the_params_are_base64_encoded() {
        let url = build_version_api_request_url_for_version("0.1.0", "x86_64", "linux");
        let params = url.split("q=").collect::<Vec<&str>>()[1];
        // params only contains web-safe base64 characters
        assert_eq!(
            params.len(),
            params
                .chars()
                .filter(|c| c.is_ascii_alphabetic()
                    || c.is_ascii_digit()
                    || *c == '_'
                    || *c == '-'
                    || *c == '=')
                .count()
        );
    }

    #[test]
    fn different_versions_give_different_urls() {
        let url1 = build_version_api_request_url_for_version("0.1.0", "x86_64", "linux");
        let url2 = build_version_api_request_url_for_version("0.1.1", "x86_64", "linux");
        assert_ne!(url1, url2);
    }

    #[test]
    fn latest_channel_and_specific_versions_have_different_urls() {
        let url1 = build_version_api_request_url_for_latest("x86_64", "linux");
        let url2 = build_version_api_request_url_for_version("0.1.1", "x86_64", "linux");
        assert_ne!(url1, url2);
    }

    #[test]
    fn different_architectures_give_different_urls() {
        let url1 = build_version_api_request_url_for_version("0.1.0", "x86_64", "linux");
        let url2 = build_version_api_request_url_for_version("0.1.0", "aarch64", "linux");
        assert_ne!(url1, url2);
    }

    #[test]
    fn different_operating_systems_give_different_urls() {
        let url1 = build_version_api_request_url_for_version("0.1.0", "x86_64", "linux");
        let url2 = build_version_api_request_url_for_version("0.1.0", "x86_64", "macos");
        assert_ne!(url1, url2);
    }
}
