// This file contains code that is impure and cannot be tested.

use crate::{context::Context, errors::CliError, thor::get_thor_binary_directory};
use protos::version::GetVersionDownloadInfoResponse;
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
                return Err(CliError::InvalidThorVersion(version.clone()));
            }
        }
        None => {
            vio.print(DETERMINING_LATEST_VERSION_MESSAGE);
        }
    }
    let download_info = fetch_download_info(version).await?;
    vio.println(format!(
        "Downloading version {}...",
        download_info.version_number
    ));
    download_and_extract_binary(context, &download_info).await?;
    Ok(download_info.version_number)
}

async fn fetch_download_info(
    _version: Option<String>,
) -> Result<GetVersionDownloadInfoResponse, CliError> {
    #[cfg(not(test))]
    {
        use crate::version_api::{
            build_version_api_request_url_for_latest, build_version_api_request_url_for_version,
        };
        use prost::Message;
        use protos::{
            decode_base_64_to_bytes, version::validate_get_version_download_info_response,
        };
        use std::env::consts::ARCH;
        use std::env::consts::OS;

        let url = match _version {
            Some(version) => build_version_api_request_url_for_version(version.as_str(), ARCH, OS),
            None => build_version_api_request_url_for_latest(ARCH, OS),
        };

        let body = reqwest::get(url)
            .await
            .map_err(|e| CliError::NetworkError(e.to_string()))?
            .text()
            .await
            .map_err(|e| CliError::NetworkError(e.to_string()))?;
        let body_bytes = decode_base_64_to_bytes(&body);
        let response = GetVersionDownloadInfoResponse::decode(body_bytes.as_slice())
            .map_err(CliError::DownloadInfoResponseDecodeError)?;
        validate_get_version_download_info_response(&response)
            .map_err(CliError::DownloadInfoResponseError)?;
        Ok(response)
    }
    #[cfg(test)]
    {
        use protos::version::{Checksum, HashFunction};

        let mut response = GetVersionDownloadInfoResponse {
            version_number: "0.1.0".into(),
            ..Default::default()
        };
        response
            .download_urls
            .push("https://downloads.buri-lang.dev".into());

        let mut checksum = Checksum::default();
        checksum.set_hash_function(HashFunction::Sha256);
        checksum.checksum = "deadbeef".into();
        response.checksum = Some(checksum);

        Ok(response)
    }
}

async fn download_and_extract_binary(
    context: &Context,
    download_info: &GetVersionDownloadInfoResponse,
) -> Result<(), CliError> {
    let thor_directory = get_thor_binary_directory(context, &download_info.version_number);
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

        if download_info.download_urls.is_empty() {
            return Err(CliError::NoDownloadUrls);
        }

        let url = download_info.download_urls[0].clone();

        let temporary_directory = Builder::new()
            .prefix("thor")
            .tempdir()
            .map_err(|e| CliError::TemporaryDirectoryCreationError(e.to_string()))?;

        let response = reqwest::get(url)
            .await
            .map_err(|e| CliError::NetworkError(e.to_string()))?;

        let mut tarball_file = {
            let file_name = response
                .url()
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .unwrap_or("tmp.bin");

            let fname = temporary_directory.path().join(file_name);

            File::create(fname).map_err(|e| CliError::CreateTarballFileError(e.to_string()))?
        };

        let bytes = response
            .bytes()
            .await
            .map_err(|e| CliError::NetworkError(e.to_string()))?;

        validate_checksum(&bytes, download_info)?;

        tarball_file
            .write_all(&bytes)
            .map_err(|e| CliError::WriteToTarballFileError(e.to_string()))?;

        let reader = BufReader::new(tarball_file);
        let tar = GzDecoder::new(reader);
        let mut archive = Archive::new(tar);
        archive
            .unpack(&temporary_directory)
            .map_err(|e| CliError::UnpackTarballError(e.to_string()))?;

        let temporary_thor_binary = temporary_directory.path().join("thor");

        let thor_binary = get_thor_binary_path(context, &download_info.version_number);

        std::fs::rename(
            temporary_thor_binary.to_str().unwrap(),
            thor_binary.as_str(),
        )
        .map_err(|e| CliError::RenameThorBinaryError(e.to_string()))?;

        // Mark it as executable
        let mut perms = std::fs::metadata(thor_binary.as_str())
            .map_err(|e| CliError::CannotReadThorBinaryPermissions(e.to_string()))?
            .permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(thor_binary.as_str(), perms)
            .map_err(|e| CliError::CannotMarkBinaryAsExecutable(e.to_string()))?;

        // Cleanup temporary files
        std::fs::remove_dir_all(temporary_directory)
            .map_err(|e| CliError::CannotRemoveTemporaryFiles(e.to_string()))?;

        Ok(())
    }
    #[cfg(test)]
    {
        use crate::thor::get_thor_binary_path;
        get_thor_binary_path(context, &download_info.version_number)
            .create_file()
            .unwrap();
        Ok(())
    }
}
