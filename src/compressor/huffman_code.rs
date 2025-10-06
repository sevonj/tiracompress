/// Huffman code format used for lookup table during encoding & decoding.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct HuffmanCode {
    len: u8,   // Number of bits used.
    bits: u32, // Leftmost bits get truncated.
}

impl HuffmanCode {
    pub const fn len(&self) -> u8 {
        self.len
    }

    pub const fn bits(&self) -> u32 {
        self.bits
    }

    pub fn new(len: u8, bits: u32) -> Self {
        debug_assert!(len <= 32);
        let num_unused_bits = 32 - len;
        let bits = (bits << num_unused_bits) >> num_unused_bits;
        Self { len, bits }
    }
}

impl std::fmt::Display for HuffmanCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0len$b}", self.bits, len = self.len as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Make sure that unused bits are scrubbed during construction, so that Eq can be derived.
    #[test]
    fn test_equal() {
        let a = HuffmanCode::new(3, 0b_101);
        let b = HuffmanCode::new(3, 0b_101);
        assert_eq!(a, b);

        let a = HuffmanCode::new(3, 0b_101);
        let b = HuffmanCode::new(3, 0b_1101);
        assert_eq!(a, b);

        let a = HuffmanCode::new(2, 0b_101);
        let b = HuffmanCode::new(2, 0b_1101);
        assert_eq!(a, b);
    }

    /// Just make sure it prints right.
    #[test]
    fn test_display() {
        let code = HuffmanCode::new(3, 0b_101);
        assert_eq!(&code.to_string(), "101");

        let code = HuffmanCode::new(4, 0b_101);
        assert_eq!(&code.to_string(), "0101");

        let code = HuffmanCode::new(5, 0b_101);
        assert_eq!(&code.to_string(), "00101");

        let code = HuffmanCode::new(2, 0b_101);
        assert_eq!(&code.to_string(), "01");

        let code = HuffmanCode::new(2, 0b_11);
        assert_eq!(&code.to_string(), "11");
    }
}
