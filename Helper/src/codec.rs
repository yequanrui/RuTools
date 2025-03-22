use base64::prelude::*;

/// 将字符串编码为Base64
/// # 参数
/// - `input`: 要编码的字符串
/// # 返回
/// 编码后的Base64字符串
pub fn encoder(input: &str) -> String {
    BASE64_STANDARD.encode(&input.as_bytes())
}

/// 将Base64字符串解码为原始字符串
/// # 参数
/// - `encoded`: 要解码的Base64字符串
/// # 返回
/// 解码后的原始字符串
pub fn decoder(encoded: &str) -> String {
    String::from_utf8(BASE64_STANDARD.decode(&encoded).unwrap()).unwrap()
}
