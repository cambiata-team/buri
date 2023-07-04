use crate::errors::CliError;
use protos::version::{Checksum, GetVersionDownloadInfoResponse, HashFunction};
use sha2::{Digest, Sha256};

fn select_checksum(download_info: &GetVersionDownloadInfoResponse) -> Result<Checksum, CliError> {
    if let Some(checksum) = &download_info.checksum {
        if checksum.hash_function == HashFunction::Sha256 as i32 {
            return Ok(checksum.clone());
        }
    }

    Err(CliError::NoSupportedChecksum)
}

pub fn validate_checksum(
    bytes: &[u8],
    download_info: &GetVersionDownloadInfoResponse,
) -> Result<(), CliError> {
    let checksum = select_checksum(download_info)?;

    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let hashed_result = hasher.finalize();
    let checksum_bytes =
        hex::decode(checksum.checksum).map_err(|e| CliError::ChecksumNotValidHex(e.to_string()))?;
    if hashed_result.as_slice() != checksum_bytes.as_slice() {
        // Expected, Actual
        return Err(CliError::ChecksumsDoNotMatch(
            hex::encode(checksum_bytes),
            hex::encode(hashed_result),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn selects_sha256_checksum() {
        let mut download_info = GetVersionDownloadInfoResponse::default();
        let mut checksum = Checksum::default();
        checksum.set_hash_function(HashFunction::Sha256);
        checksum.checksum = "checksum".to_string();
        download_info.checksum = Some(checksum.clone());
        let selected_checksum = select_checksum(&download_info).unwrap();
        assert_eq!(selected_checksum, checksum);
    }

    #[test]
    fn errors_if_not_checksum_found() {
        let download_info = GetVersionDownloadInfoResponse::default();
        let selected_checksum = select_checksum(&download_info);
        assert!(selected_checksum.is_err());
    }

    #[test]
    fn validates_checksum() {
        let mut download_info = GetVersionDownloadInfoResponse::default();
        let mut checksum = Checksum::default();
        checksum.set_hash_function(HashFunction::Sha256);
        checksum.checksum =
            "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08".to_string();
        download_info.checksum = Some(checksum.clone());
        let bytes = "test".as_bytes();
        let result = validate_checksum(bytes, &download_info);
        assert!(result.is_ok());
    }

    #[test]
    fn invalid_checksum_errors() {
        let mut download_info = GetVersionDownloadInfoResponse::default();
        let mut checksum = Checksum::default();
        checksum.set_hash_function(HashFunction::Sha256);
        checksum.checksum = "beef".to_string();
        download_info.checksum = Some(checksum.clone());
        let bytes = "test".as_bytes();
        let result = validate_checksum(bytes, &download_info);
        assert_eq!(
            result,
            Err(CliError::ChecksumsDoNotMatch(
                "beef".into(),
                "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08".into()
            ))
        );
    }

    #[test]
    fn checksum_with_invalid_characters_errors() {
        let mut download_info = GetVersionDownloadInfoResponse::default();
        let mut checksum = Checksum::default();
        checksum.set_hash_function(HashFunction::Sha256);
        checksum.checksum = "I am definitely not a checksum".to_string();
        download_info.checksum = Some(checksum.clone());
        let bytes = "test".as_bytes();
        let result = validate_checksum(bytes, &download_info);
        assert!(matches!(result, Err(CliError::ChecksumNotValidHex(_))));
    }
}
