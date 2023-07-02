use protos::version::{
    get_version_download_info_request, GetVersionDownloadInfoRequest, VersionInfo,
};

pub fn does_version_info_match_request(
    request: &GetVersionDownloadInfoRequest,
    version_info: &VersionInfo,
) -> bool {
    if request.program() != version_info.program() {
        return false;
    }

    if request.architecture() != version_info.architecture() {
        return false;
    }

    if request.operating_system_family() != version_info.operating_system_family() {
        return false;
    }

    if let Some(get_version_download_info_request::Version::VersionNumber(version)) =
        &request.version
    {
        let version_number = match version_info.version_number {
            Some(ref version_number) => version_number,
            None => return false,
        };
        if version != version_number {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {
    use protos::version::{Channel, Program, SemanticVersion};

    use super::*;

    fn create_version_info() -> VersionInfo {
        let mut version_info = VersionInfo::default();
        version_info.set_program(protos::version::Program::VersionManager);
        version_info.set_architecture(protos::version::Architecture::Arm64);
        version_info.set_operating_system_family(protos::version::OperatingSystemFamily::Darwin);
        version_info.version_number = Some(SemanticVersion {
            major: Some(1),
            minor: Some(2),
            patch: Some(3),
        });

        version_info
    }

    fn create_request() -> GetVersionDownloadInfoRequest {
        let mut request = GetVersionDownloadInfoRequest::default();
        request.set_program(protos::version::Program::VersionManager);
        request.set_architecture(protos::version::Architecture::Arm64);
        request.set_operating_system_family(protos::version::OperatingSystemFamily::Darwin);
        request.version = Some(get_version_download_info_request::Version::VersionNumber(
            SemanticVersion {
                major: Some(1),
                minor: Some(2),
                patch: Some(3),
            },
        ));
        request
    }

    #[test]
    fn returns_true_if_version_info_matches_request() {
        assert!(does_version_info_match_request(
            &create_request(),
            &create_version_info()
        ));
    }

    #[test]
    fn returns_false_if_program_does_not_match() {
        let mut request = create_request();
        let mut version_info = create_version_info();
        request.set_program(Program::Thor);
        version_info.set_program(Program::VersionManager);
        assert!(!does_version_info_match_request(&request, &version_info));
    }

    #[test]
    fn returns_false_if_architecture_does_not_match() {
        let mut request = create_request();
        let mut version_info = create_version_info();
        request.set_architecture(protos::version::Architecture::X8664);
        version_info.set_architecture(protos::version::Architecture::Arm64);
        assert!(!does_version_info_match_request(&request, &version_info));
    }

    #[test]
    fn returns_false_if_operating_system_family_does_not_match() {
        let mut request = create_request();
        let mut version_info = create_version_info();
        request.set_operating_system_family(protos::version::OperatingSystemFamily::Linux);
        version_info.set_operating_system_family(protos::version::OperatingSystemFamily::Darwin);
        assert!(!does_version_info_match_request(&request, &version_info));
    }

    #[test]
    fn returns_false_if_version_number_does_not_match() {
        let mut request = create_request();
        let mut version_info = create_version_info();
        request.version = Some(get_version_download_info_request::Version::VersionNumber(
            SemanticVersion {
                major: Some(4),
                minor: Some(5),
                patch: Some(6),
            },
        ));
        version_info.version_number = Some(SemanticVersion {
            major: Some(1),
            minor: Some(2),
            patch: Some(3),
        });
        assert!(!does_version_info_match_request(&request, &version_info));
    }

    #[test]
    fn returns_true_if_request_asks_for_channel() {
        let mut request = create_request();
        let mut version_info = create_version_info();
        request.version = Some(get_version_download_info_request::Version::Channel(
            Channel::Latest.into(),
        ));
        version_info.version_number = None;
        version_info.version_number = Some(SemanticVersion {
            major: Some(1),
            minor: Some(2),
            patch: Some(3),
        });
        assert!(does_version_info_match_request(&request, &version_info));
    }
}
