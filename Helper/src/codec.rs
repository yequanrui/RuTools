use base64::prelude::*;

pub fn encoder(original_string: &str) -> String {
    BASE64_STANDARD.encode(&original_string.as_bytes())
}

pub fn decoder(encoded_string: &str) -> String {
    String::from_utf8(BASE64_STANDARD.decode(&encoded_string).unwrap()).unwrap()
}
