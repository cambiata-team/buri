// This file contains code that is impure and cannot be tested.

use crate::{context::Context, thor::get_thor_binary_directory, CliError};
use virtual_io::VirtualIo;

pub const DETERMINING_LATEST_VERSION_MESSAGE: &str =
    "No version configured. Determining latest version...\n";

/// Returns THor version if successful. Otherwise error.
pub async fn download_thor(
    context: &Context,
    vio: &mut impl VirtualIo,
    version: Option<String>,
) -> Result<String, CliError> {
    if version.is_none() {
        vio.print(DETERMINING_LATEST_VERSION_MESSAGE);
    }
    let (version, checksum) = fetch_version_info(version).await?;
    vio.println(format!("Downloading version {}...", version));
    download_and_extract_binary(context, &version, &checksum).await?;
    // TODO: Write to config file the new version.
    Ok(version)
}

async fn fetch_version_info(version: Option<String>) -> Result<(String, String), CliError> {
    #[cfg(not(test))]
    {
        use crate::version_api::{
            build_version_api_request_url_for_latest, build_version_api_request_url_for_version,
        };
        use macros::return_if_error;
        use prost::Message;
        use protos::{
            decode_base_64_to_bytes,
            version::{validate_version_info_message, HashFunction, VersionInfo},
        };
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
        let mut supported_checksum = String::from("");
        for checksum in &version_info.checksums {
            if checksum.hash_function == HashFunction::Sha256 as i32 {
                supported_checksum = checksum.checksum.clone();
                break;
            }
        }
        if supported_checksum.is_empty() {
            return Err(CliError::InternalError);
        }
        Ok((version_info.version_number, supported_checksum))
    }
    #[cfg(test)]
    {
        Ok((
            version.unwrap_or(String::from("latest")),
            "checksum".to_string(),
        ))
    }
}

async fn download_and_extract_binary(
    context: &Context,
    version: &str,
    checksum: &str,
) -> Result<(), CliError> {
    // Ensures the directory exists.
    get_thor_binary_directory(context, version)
        .create_dir_all()
        .unwrap();

    #[cfg(not(test))]
    {
        Ok(())
    }
    #[cfg(test)]
    {
        use crate::thor::get_thor_binary_path;
        get_thor_binary_path(context, version)
            .create_file()
            .unwrap();
        Ok(())
    }
}
