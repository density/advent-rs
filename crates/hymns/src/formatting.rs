const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";

#[must_use]
pub fn byte_to_hex(byte: u8) -> [u8; 2] {
    [
        HEX_CHARS[(byte >> 4) as usize],
        HEX_CHARS[(byte & 0x0F) as usize],
    ]
}

pub fn to_hex_array(bytes: &[u8], output: &mut [u8]) {
    debug_assert_eq!(output.len(), bytes.len() * 2);

    for (i, &byte) in bytes.iter().enumerate() {
        let hex = byte_to_hex(byte);
        output[2 * i] = hex[0];
        output[2 * i + 1] = hex[1];
    }
}

// tests
#[cfg(test)]
mod tests {
    #[test]
    fn test_to_hex_array() {}
}
