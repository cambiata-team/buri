use protos::version::{Architecture, OperatingSystemFamily, Program};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Asset<'a> {
    pub name: &'a str,
    pub browser_download_url: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct Release<'a> {
    pub tag_name: &'a str,
    pub assets: Vec<Asset<'a>>,
}

pub struct BinaryInfo {
    pub program: Program,
    pub architecture: Architecture,
    pub operating_system_family: OperatingSystemFamily,
    pub download_url: String,
    pub hash_download_url: String,
}

pub fn get_hash_from_sha256_file(sha256_file: &str) -> &str {
    // A Sha256 hash is always 64 characters long in base64.
    &sha256_file[0..64]
}

pub fn parse_asset_to_binary_info(asset: &Asset) -> Option<BinaryInfo> {
    let file_name = asset.name;
    if file_name.ends_with("sha256") {
        return None;
    }
    let program = match file_name {
        x if x.starts_with("cli") => Program::VersionManager,
        x if x.starts_with("thor") => Program::Thor,
        _ => return None,
    };

    let architecture = match file_name {
        x if x.contains("aarch64") => Architecture::Arm64,
        x if x.contains("x86_64") => Architecture::X8664,
        _ => return None,
    };

    let operating_system_family = match file_name {
        x if x.contains("darwin") => OperatingSystemFamily::Darwin,
        x if x.contains("linux") => OperatingSystemFamily::Linux,
        _ => return None,
    };

    Some(BinaryInfo {
        program,
        architecture,
        operating_system_family,
        download_url: asset.browser_download_url.to_string(),
        hash_download_url: get_hash_download_url_from_binary_download_url(
            asset.browser_download_url,
        ),
    })
}

fn get_hash_download_url_from_binary_download_url(binary_download_url: &str) -> String {
    binary_download_url
        .replace(".tar.gz", ".sha256")
        .replace(".zip", ".sha256")
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::raw_data::{RAW_RELEASE_DATA, SHA_256_FILE};

    #[test]
    fn can_deserialize_tag_name() {
        let release: Release = serde_json::from_str(RAW_RELEASE_DATA).unwrap();
        assert_eq!(release.tag_name, "0.2.0");
    }

    #[test]
    fn can_deserialize_release_data() {
        let release: Release = serde_json::from_str(RAW_RELEASE_DATA).unwrap();
        assert_eq!(release.assets.len(), 10);
        assert_eq!(release.assets[0].name, "cli-aarch64-apple-darwin.sha256");
        assert_eq!(
            release.assets[0].browser_download_url,
            "https://github.com/cambiata-team/buri/releases/download/0.2.0/cli-aarch64-apple-darwin.sha256",
        );
        assert_eq!(release.assets[1].name, "cli-aarch64-apple-darwin.tar.gz");
        assert_eq!(
            release.assets[1].browser_download_url,
            "https://github.com/cambiata-team/buri/releases/download/0.2.0/cli-aarch64-apple-darwin.tar.gz",
        );
    }

    #[test]
    fn can_get_hash_from_sha256_file() {
        assert_eq!(
            get_hash_from_sha256_file(SHA_256_FILE),
            "dc06fa9d945ac660dd095a92032cc207cdf94f8864d074cb83d9e1606f5f5a0b"
        );
    }

    #[test]
    fn parses_program() {
        let version_manager = parse_asset_to_binary_info(&Asset {
            name: "cli-aarch64-apple-darwin.tar.gz",
            ..Default::default()
        })
        .unwrap();
        let thor = parse_asset_to_binary_info(&Asset {
            name: "thor-aarch64-apple-darwin.tar.gz",
            ..Default::default()
        })
        .unwrap();
        assert_eq!(version_manager.program, Program::VersionManager);
        assert_eq!(thor.program, Program::Thor);
    }

    #[test]
    fn parses_architecture() {
        let arm64 = parse_asset_to_binary_info(&Asset {
            name: "cli-aarch64-apple-darwin.tar.gz",
            ..Default::default()
        })
        .unwrap();
        let x86_64 = parse_asset_to_binary_info(&Asset {
            name: "cli-x86_64-apple-darwin.tar.gz",
            ..Default::default()
        })
        .unwrap();
        assert_eq!(arm64.architecture, Architecture::Arm64);
        assert_eq!(x86_64.architecture, Architecture::X8664);
    }

    #[test]
    fn parses_operating_system_family() {
        let darwin = parse_asset_to_binary_info(&Asset {
            name: "cli-aarch64-apple-darwin.tar.gz",
            ..Default::default()
        })
        .unwrap();
        let linux = parse_asset_to_binary_info(&Asset {
            name: "cli-x86_64-unknown-linux-gnu.tar.gz",
            ..Default::default()
        })
        .unwrap();
        assert_eq!(
            darwin.operating_system_family,
            OperatingSystemFamily::Darwin
        );
        assert_eq!(linux.operating_system_family, OperatingSystemFamily::Linux);
    }

    #[test]
    fn determines_hash_download_url_from_binary_download_url() {
        let tar_binary_download_url =
            "https://github.com/cambiata-team/buri/releases/download/0.2.0/cli-aarch64-apple-darwin.tar.gz";
        let tar_hash_download_url =
            get_hash_download_url_from_binary_download_url(tar_binary_download_url);
        assert_eq!(
            tar_hash_download_url,
            "https://github.com/cambiata-team/buri/releases/download/0.2.0/cli-aarch64-apple-darwin.sha256"
        );

        let zip_binary_download_url =
            "https://github.com/cambiata-team/buri/releases/download/0.2.0/cli-aarch64-apple-darwin.zip";
        let zip_hash_download_url =
            get_hash_download_url_from_binary_download_url(zip_binary_download_url);
        assert_eq!(
            zip_hash_download_url,
            "https://github.com/cambiata-team/buri/releases/download/0.2.0/cli-aarch64-apple-darwin.sha256"
        );
    }

    #[test]
    fn parse_asset_to_binary_info_preserves_download_url() {
        let asset = Asset {
            name: "cli-aarch64-apple-darwin.tar.gz",
            browser_download_url: "example.com",
        };
        let binary_info = parse_asset_to_binary_info(&asset).unwrap();
        assert_eq!(binary_info.download_url, "example.com");
    }

    #[test]
    fn parse_asset_to_binary_info_creates_hash_url() {
        let asset = Asset {
            name: "cli-aarch64-apple-darwin.tar.gz",
            browser_download_url: "https://github.com/cambiata-team/buri/releases/download/0.2.0/cli-aarch64-apple-darwin.tar.gz",
        };
        let binary_info = parse_asset_to_binary_info(&asset).unwrap();
        assert_eq!(
            binary_info.hash_download_url,
            "https://github.com/cambiata-team/buri/releases/download/0.2.0/cli-aarch64-apple-darwin.sha256"
        );
    }
}
