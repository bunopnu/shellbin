/// Converts a UTF-8 encoded string into a vector of UTF-16 encoded bytes (wide characters).
pub fn to_wide_bytes(source: String) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(source.len() * 2);

    for character in source.encode_utf16() {
        bytes.push((character & 0xFF) as u8);
        bytes.push((character >> 8) as u8);
    }

    bytes
}
