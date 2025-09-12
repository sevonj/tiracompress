use std::io::Read;
use std::io::Write;

/// The serializable huffman code format.
#[derive(Debug, PartialEq, Eq)]
pub struct HuffmanCode {
    len: u8,   // Number of bits used.
    code: u32, // The bits. Most significant bits get truncated.
}

impl HuffmanCode {
    pub fn new(len: u8, code: u32) -> Self {
        assert!(len <= 32);
        let num_unused_bits = 32 - len;
        let code = (code << num_unused_bits) >> num_unused_bits;
        Self { len, code }
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<Self, std::io::Error> {
        todo!()
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        todo!()
    }
}

impl std::fmt::Display for HuffmanCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0len$b}", self.code, len = self.len as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Make sure that unused bits are scrubbed during construction.
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
