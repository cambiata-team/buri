// This file contains code that is impure and cannot be tested.

use crate::{context::Context, errors::CliError};
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
    println!("hello");
    #[cfg(not(test))]
    {
        use crate::security::validate_checksum;
        use crate::thor::get_thor_binary_binary_pathbuf;
        use flate2::bufread::GzDecoder;
        use std::io::BufReader;
        use tar::Archive;

        if download_info.download_urls.is_empty() {
            return Err(CliError::NoDownloadUrls);
        }

        let url = download_info.download_urls[0].clone();

        let response = reqwest::get(url)
            .await
            .map_err(|e| CliError::NetworkError(e.to_string()))?;

        let bytes = response
            .bytes()
            .await
            .map_err(|e| CliError::NetworkError(e.to_string()))?;

        validate_checksum(&bytes, download_info)?;

        let thor_binary_path = get_thor_binary_binary_pathbuf(&download_info.version_number);
        let reader = BufReader::new(&bytes[..]);
        let tar = GzDecoder::new(reader);
        let mut archive = Archive::new(tar);
        archive.set_preserve_permissions(true);
        archive.set_overwrite(true);
        for file in archive.entries().unwrap() {
            let mut file = file.unwrap();
            file.unpack(&thor_binary_path).unwrap();
        }

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
