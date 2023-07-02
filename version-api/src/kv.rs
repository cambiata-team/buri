use protos::{
    encode_message_to_base_64,
    version::{
        get_version_download_info_request, version_info_key, Channel, Checksum,
        GetVersionDownloadInfoRequest, HashFunction, SemanticVersion, VersionInfo, VersionInfoKey,
    },
};

use crate::parse_release::BinaryInfo;

pub fn create_version_info_key_from_request(request: &GetVersionDownloadInfoRequest) -> String {
    let mut key = VersionInfoKey::default();
    key.set_program(request.program());
    key.set_architecture(request.architecture());
    key.set_operating_system_family(request.operating_system_family());
    key.version = match &request.version {
        Some(get_version_download_info_request::Version::Channel(channel)) => {
            Some(version_info_key::Version::Channel(*channel))
        }
        Some(get_version_download_info_request::Version::VersionNumber(version)) => {
            Some(version_info_key::Version::VersionNumber(version.clone()))
        }
        None => panic!("Check the request is valid before calling this function"),
    };

    encode_message_to_base_64(&key)
}

pub fn get_keys_from_binary_info(info: &BinaryInfo, version: &SemanticVersion) -> (String, String) {
    let mut latest_key = VersionInfoKey::default();
    latest_key.set_program(info.program);
    latest_key.set_architecture(info.architecture);
    latest_key.set_operating_system_family(info.operating_system_family);
    latest_key.version = Some(protos::version::version_info_key::Version::Channel(
        Channel::Latest.into(),
    ));
    let mut version_key = latest_key.clone();
    version_key.version = Some(protos::version::version_info_key::Version::VersionNumber(
        version.clone(),
    ));
    (
        encode_message_to_base_64(&latest_key),
        encode_message_to_base_64(&version_key),
    )
}

pub fn create_version_info(
    info: &BinaryInfo,
    version: &SemanticVersion,
    sha256: &str,
) -> VersionInfo {
    let mut version_info = VersionInfo::default();
    version_info.set_program(info.program);
    version_info.set_architecture(info.architecture);
    version_info.set_operating_system_family(info.operating_system_family);
    version_info.version_number = Some(version.clone());
    let mut checksum = Checksum::default();
    checksum.set_hash_function(HashFunction::Sha256);
    checksum.checksum = sha256.to_string();
    version_info.checksums = vec![checksum];
    version_info.download_urls = vec![info.download_url.clone()];
    version_info
}

#[cfg(test)]
mod test {
    use super::*;
    use protos::version::{
        validate_version_info_message, Architecture, Channel, OperatingSystemFamily, Program,
        SemanticVersion,
    };

    fn create_request() -> GetVersionDownloadInfoRequest {
        let mut request = GetVersionDownloadInfoRequest::default();
        request.set_program(Program::VersionManager);
        request.set_architecture(Architecture::Arm64);
        request.set_operating_system_family(OperatingSystemFamily::Darwin);
        request.version = Some(get_version_download_info_request::Version::Channel(
            Channel::Latest.into(),
        ));
        request
    }

    #[test]
    fn test_create_version_info_key_from_request() {
        let key = create_version_info_key_from_request(&create_request());
        assert_eq!(key, "CAIQASACKAI=");
    }

    #[test]
    fn different_programs_produce_different_keys() {
        let mut version_manager_request = create_request();
        let mut thor_request = create_request();
        version_manager_request.set_program(Program::VersionManager);
        thor_request.set_program(Program::Thor);
        assert_ne!(
            create_version_info_key_from_request(&version_manager_request),
            create_version_info_key_from_request(&thor_request)
        );
    }

    #[test]
    fn different_architectures_produce_different_keys() {
        let mut arm64_request = create_request();
        let mut x86_64_request = create_request();
        arm64_request.set_architecture(Architecture::Arm64);
        x86_64_request.set_architecture(Architecture::X8664);
        assert_ne!(
            create_version_info_key_from_request(&arm64_request),
            create_version_info_key_from_request(&x86_64_request)
        );
    }

    #[test]
    fn different_operating_system_families_produce_different_keys() {
        let mut darwin_request = create_request();
        let mut linux_request = create_request();
        darwin_request.set_operating_system_family(OperatingSystemFamily::Darwin);
        linux_request.set_operating_system_family(OperatingSystemFamily::Linux);
        assert_ne!(
            create_version_info_key_from_request(&darwin_request),
            create_version_info_key_from_request(&linux_request)
        );
    }

    #[test]
    fn different_versions_produce_different_keys() {
        let mut latest_request = create_request();
        let mut v1_request = create_request();
        latest_request.version = Some(get_version_download_info_request::Version::Channel(
            Channel::Latest.into(),
        ));
        v1_request.version = Some(get_version_download_info_request::Version::VersionNumber(
            SemanticVersion {
                major: Some(1),
                minor: Some(0),
                patch: Some(0),
            },
        ));
        assert_ne!(
            create_version_info_key_from_request(&latest_request),
            create_version_info_key_from_request(&v1_request)
        );
    }

    #[test]
    fn different_semantic_versions_produce_different_keys() {
        let mut v3_request = create_request();
        let mut v1_request = create_request();
        v3_request.version = Some(get_version_download_info_request::Version::VersionNumber(
            SemanticVersion {
                major: Some(3),
                minor: Some(1),
                patch: Some(4),
            },
        ));
        v1_request.version = Some(get_version_download_info_request::Version::VersionNumber(
            SemanticVersion {
                major: Some(1),
                minor: Some(0),
                patch: Some(0),
            },
        ));
        assert_ne!(
            create_version_info_key_from_request(&v1_request),
            create_version_info_key_from_request(&v3_request),
        );
    }

    #[test]
    fn test_get_keys_from_binary_info() {
        let info = BinaryInfo {
            program: Program::VersionManager,
            architecture: Architecture::Arm64,
            operating_system_family: OperatingSystemFamily::Darwin,
            download_url: "https://downloads.buri-lang.dev".to_string(),
            hash_download_url: "https://downloads.buri-lang.dev/hash".to_string(),
        };
        let version = SemanticVersion {
            major: Some(1),
            minor: Some(0),
            patch: Some(0),
        };
        let (latest_key, version_key) = get_keys_from_binary_info(&info, &version);
        assert_eq!(latest_key, "CAIQASACKAI=");
        assert_eq!(version_key, "CAIaBggBEAAYACACKAI=");
    }

    #[test]
    fn test_create_version_info() {
        let info = BinaryInfo {
            program: Program::VersionManager,
            architecture: Architecture::Arm64,
            operating_system_family: OperatingSystemFamily::Darwin,
            download_url: "https://downloads.buri-lang.dev".to_string(),
            hash_download_url: "https://downloads.buri-lang.dev/hash".to_string(),
        };
        let version = SemanticVersion {
            major: Some(1),
            minor: Some(0),
            patch: Some(0),
        };
        let sha256 = "1234567890abcdef";
        let version_info = create_version_info(&info, &version, sha256);
        assert!(validate_version_info_message(&version_info).is_ok());
    }
}
