// This file contains code that is impure and cannot be tested.

use crate::{context::Context, thor::get_thor_binary_directory, CliError};
use protos::version::VersionInfo;
use version::is_valid_version;
use virtual_io::VirtualIo;

pub const DETERMINING_LATEST_VERSION_MESSAGE: &str =
    "No version configured. Determining latest version...\n";

/// Returns THor version if successful. Otherwise error.
pub async fn download_thor(
    context: &Context,
    vio: &mut impl VirtualIo,
    version: Option<String>,
) -> Result<String, CliError> {
    match &version {
        Some(version) => {
            if !is_valid_version(version) {
                return Err(CliError::InvalidThorVersion);
            }
        }
        None => {
            vio.print(DETERMINING_LATEST_VERSION_MESSAGE);
        }
    }
    let version_info = fetch_version_info(version).await?;
    vio.println(format!(
        "Downloading version {}...",
        version_info.version_number
    ));
    download_and_extract_binary(context, &version_info).await?;
    Ok(version_info.version_number)
}

async fn fetch_version_info(version: Option<String>) -> Result<VersionInfo, CliError> {
    #[cfg(not(test))]
    {
        use crate::version_api::{
            build_version_api_request_url_for_latest, build_version_api_request_url_for_version,
        };
        use macros::return_if_error;
        use prost::Message;
        use protos::{decode_base_64_to_bytes, version::validate_version_info_message};
        use std::env::consts::ARCH;
        use std::env::consts::OS;

        let url = match version {
            Some(version) => build_version_api_request_url_for_version(version.as_str(), ARCH, OS),
            None => build_version_api_request_url_for_latest(ARCH, OS),
        };

        let body = reqwest::get(url)
            .await
            .map_err(|_| CliError::NetworkError)?
            .text()
            .await
            .map_err(|_| CliError::NetworkError)?;
        let body_bytes = decode_base_64_to_bytes(&body);
        let version_info = return_if_error!(
            VersionInfo::decode(body_bytes.as_slice()),
            Err(CliError::InternalError)
        );
        return_if_error!(
            validate_version_info_message(&version_info),
            Err(CliError::InternalError)
        );
        Ok(version_info)
    }
    #[cfg(test)]
    {
        use protos::version::{Checksum, HashFunction};

        let mut checksum = Checksum::default();
        checksum.set_hash_function(HashFunction::Sha256);
        checksum.checksum = "checksum".to_string();

        let info = VersionInfo {
            version_number: version.unwrap_or("latest".to_string()),
            checksums: vec![checksum],
            ..Default::default()
        };
        Ok(info)
    }
}

async fn download_and_extract_binary(
    context: &Context,
    version_info: &VersionInfo,
) -> Result<(), CliError> {
    let thor_directory = get_thor_binary_directory(context, &version_info.version_number);
    // Ensures the thor directory exists.
    thor_directory.create_dir_all().unwrap();

    #[cfg(not(test))]
    {
        use crate::security::validate_checksum;
        use crate::thor::get_thor_binary_path;
        use flate2::bufread::GzDecoder;
        use std::fs::File;
        use std::io::prelude::*;
        use std::io::BufReader;
        use std::os::unix::prelude::PermissionsExt;
        use tar::Archive;
        use tempfile::Builder;

        if version_info.download_urls.is_empty() {
            return Err(CliError::InternalError);
        }

        let url = version_info.download_urls[0].clone();

        let temporary_directory = Builder::new()
            .prefix("thor")
            .tempdir()
            .map_err(|_| CliError::InternalError)?;

        let response = reqwest::get(url)
            .await
            .map_err(|_| CliError::NetworkError)?;

        let mut tarball_file = {
            let file_name = response
                .url()
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .unwrap_or("tmp.bin");

            let fname = temporary_directory.path().join(file_name);

            File::create(fname).map_err(|_| CliError::InternalError)?
        };

        let bytes = response.bytes().await.map_err(|_| CliError::NetworkError)?;

        validate_checksum(&bytes, version_info)?;

        tarball_file
            .write_all(&bytes)
            .map_err(|_| CliError::InternalError)?;

        let reader = BufReader::new(tarball_file);
        let tar = GzDecoder::new(reader);
        let mut archive = Archive::new(tar);
        archive
            .unpack(&temporary_directory)
            .map_err(|_| CliError::InternalError)?;

        let temporary_thor_binary = temporary_directory.path().join("thor");

        let thor_binary = get_thor_binary_path(context, &version_info.version_number);

        std::fs::rename(
            temporary_thor_binary.to_str().unwrap(),
            thor_binary.as_str(),
        )
        .map_err(|_| CliError::InternalError)?;

        // Mark it as executable
        let mut perms = std::fs::metadata(thor_binary.as_str())
            .map_err(|_| CliError::InternalError)?
            .permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(thor_binary.as_str(), perms)
            .map_err(|_| CliError::InternalError)?;

        // Cleanup temporary files
        std::fs::remove_dir_all(temporary_directory).map_err(|_| CliError::InternalError)?;

        Ok(())
    }
    #[cfg(test)]
    {
        use crate::thor::get_thor_binary_path;
        get_thor_binary_path(context, &version_info.version_number)
            .create_file()
            .unwrap();
        Ok(())
    }
}
