use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine,
};
use prost::Message;

pub mod version;

const ENGINE: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::PAD);

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
