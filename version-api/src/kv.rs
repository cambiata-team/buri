use protos::{
    encode_message_to_base_64,
    version::{
        get_version_download_info_request, version_info_key, GetVersionDownloadInfoRequest,
        VersionInfoKey,
    },
};

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

#[cfg(test)]
mod test {
    use super::*;
    use protos::version::{Architecture, Channel, OperatingSystemFamily, Program, SemanticVersion};

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
}
