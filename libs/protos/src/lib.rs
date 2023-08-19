use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine,
};
use prost::Message;

pub mod version;

const ENGINE: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

pub fn encode_message_to_bytes<T>(message: &T) -> Vec<u8>
where
    T: Message,
{
    let response_size = message.encoded_len();
    let mut response_buffer: Vec<u8> = Vec::with_capacity(response_size);
    message.encode(&mut response_buffer).unwrap();
    response_buffer
}

pub fn encode_message_to_base_64<T>(message: &T) -> String
where
    T: Message,
{
    let bytes = encode_message_to_bytes(message);
    ENGINE.encode(bytes)
}

pub fn decode_base_64_to_bytes(base_64: &str) -> Vec<u8> {
    ENGINE.decode(base_64).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encoding_and_decoding_from_base_64_gets_same_message() {
        let message = version::GetVersionDownloadInfoResponse {
            version_number: "1.0.0".to_string(),
            download_urls: vec!["https://example.com".to_string()],
            checksum: Some(version::Checksum {
                hash_function: version::HashFunction::Sha256 as i32,
                checksum: "1234567890".to_string(),
            }),
        };
        let encoded = encode_message_to_base_64(&message);
        let decoded = decode_base_64_to_bytes(&encoded);
        let decoded_message =
            version::GetVersionDownloadInfoResponse::decode(decoded.as_slice()).unwrap();
        assert_eq!(message, decoded_message);
    }

    #[test]
    fn encoding_and_decoding_from_bytes_gets_same_message() {
        let message = version::GetVersionDownloadInfoResponse {
            version_number: "1.0.0".to_string(),
            download_urls: vec!["https://example.com".to_string()],
            checksum: Some(version::Checksum {
                hash_function: version::HashFunction::Sha256 as i32,
                checksum: "1234567890".to_string(),
            }),
        };
        let encoded = encode_message_to_bytes(&message);
        let decoded_message =
            version::GetVersionDownloadInfoResponse::decode(encoded.as_slice()).unwrap();
        assert_eq!(message, decoded_message);
    }
}
