use std::cmp::min;
use std::collections::HashMap;
use std::io::Read;
use std::io::Write;

use super::HuffmanCode;

/// Writer for packed data
pub struct CodeWriter<W: Write> {
    writer: W,
    /// Cached incomplete byte
    cache: u8,
    /// Number of bits in cache
    cache_len: u8,
}

impl<W: Write> CodeWriter<W> {
    pub fn new(writer: W) -> Self {
        CodeWriter {
            writer,
            cache: 0,
            cache_len: 0,
        }
    }

    /// Pack a byte
    pub fn write(&mut self, code: &HuffmanCode) -> Result<(), std::io::Error> {
        let mut total_len = code.len();
        let mut bits = code.bits() << (32 - total_len);

        while total_len != 0 {
            let len = min(total_len, 8);

            let available = 8 - self.cache_len;
            let len_write = min(available, len);
            self.cache <<= len_write % 8;
            let mut value = (bits >> (32 - len_write)) as u8;
            value <<= self.cache_len;
            value >>= self.cache_len;
            self.cache |= value;
            self.cache_len += len_write;

            if self.cache_len == 8 {
                self.writer.write_all(&[self.cache])?;
                self.cache = 0;
                self.cache_len = 0;
            }

            total_len -= len_write;
            bits <<= len_write;
        }

        Ok(())
    }

    pub fn close(mut self) -> Result<(), std::io::Error> {
        if self.cache_len > 0 {
            self.cache <<= 8 - self.cache_len;
            self.writer.write_all(&[self.cache])?;
        }
        Ok(())
    }
}

/// A very simple reader for packed data
pub struct CodeReader<'a, R: Read> {
    map: &'a HashMap<HuffmanCode, u8>,
    reader: R,
    /// Cached incomplete byte
    cache: u8,
    /// Which bit is next?
    bit: u8,
}

impl<'a, R: Read> CodeReader<'a, R> {
    pub fn new(reader: R, map: &'a HashMap<HuffmanCode, u8>) -> Self {
        CodeReader {
            map,
            reader,
            cache: 0,
            bit: 7,
        }
    }

    /// Unpack a byte
    pub fn read(&mut self) -> Result<u8, std::io::Error> {
        let mut len = 0_u8;
        let mut bits = 0_u32;

        loop {
            len += 1;
            bits <<= 1;
            bits += self.next_bit()? as u32;

            if let Some(byte) = self.map.get(&HuffmanCode::new(len, bits)) {
                return Ok(*byte);
            }
        }
    }

    /// Next bit from stream. Return falue is always 0 or 1.
    fn next_bit(&mut self) -> Result<u8, std::io::Error> {
        if self.bit == 7 {
            let mut buf = [0_u8];
            self.reader.read_exact(&mut buf)?;
            self.cache = buf[0];
        }

        let value = (self.cache >> self.bit) & 1;

        if self.bit == 0 {
            self.bit = 8;
        }
        self.bit -= 1;

        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_whitebox_1b() {
        let mut compressed = vec![];
        let mut writer = CodeWriter::new(&mut compressed);

        writer.write(&HuffmanCode::new(3, 0b_111)).unwrap();
        assert_eq!(writer.cache_len, 3);
        assert_eq!(writer.cache, 0b_111);
        writer.write(&HuffmanCode::new(3, 0b_000)).unwrap();
        assert_eq!(writer.cache_len, 6);
        assert_eq!(writer.cache, 0b_111_000);
        writer.write(&HuffmanCode::new(2, 0b_11)).unwrap();
        assert_eq!(writer.cache_len, 0);
        assert_eq!(writer.cache, 0);
        writer.close().unwrap();
        assert_eq!(compressed, vec![0b_111_000_11]);
    }

    #[test]
    fn test_write_whitebox_2b() {
        let mut compressed = vec![];
        let mut writer = CodeWriter::new(&mut compressed);

        writer.write(&HuffmanCode::new(3, 0b_111)).unwrap();
        assert_eq!(writer.cache_len, 3);
        assert_eq!(writer.cache, 0b_111);

        writer.write(&HuffmanCode::new(3, 0b_111)).unwrap();
        assert_eq!(writer.cache_len, 6);
        assert_eq!(writer.cache, 0b_111_111);

        writer.write(&HuffmanCode::new(3, 0b_110)).unwrap();
        assert_eq!(writer.cache_len, 1);
        assert_eq!(writer.cache, 0b_0);

        writer.write(&HuffmanCode::new(3, 0b_000)).unwrap();
        assert_eq!(writer.cache_len, 4);
        assert_eq!(writer.cache, 0b_0_000);

        writer.write(&HuffmanCode::new(3, 0b_000)).unwrap();
        assert_eq!(writer.cache_len, 7);
        assert_eq!(writer.cache, 0b_0_000_000);

        writer.write(&HuffmanCode::new(1, 0b_0)).unwrap();
        assert_eq!(writer.cache_len, 0);
        assert_eq!(writer.cache, 0);

        writer.close().unwrap();
        assert_eq!(compressed, vec![0b_111_111_11, 0b_0_000_000_0]);
    }
    #[test]

    fn test_write_whitebox_2b_2() {
        let mut compressed = vec![];
        let mut writer = CodeWriter::new(&mut compressed);

        writer.write(&HuffmanCode::new(3, 0b_111)).unwrap();
        assert_eq!(writer.cache_len, 3);
        assert_eq!(writer.cache, 0b_111);

        writer.write(&HuffmanCode::new(3, 0b_000)).unwrap();
        assert_eq!(writer.cache_len, 6);
        assert_eq!(writer.cache, 0b_111_000);

        writer.write(&HuffmanCode::new(3, 0b_001)).unwrap();
        assert_eq!(writer.cache_len, 1);
        assert_eq!(writer.cache, 0b_1);

        writer.write(&HuffmanCode::new(3, 0b_010)).unwrap();
        assert_eq!(writer.cache_len, 4);
        assert_eq!(writer.cache, 0b_1_010);

        writer.write(&HuffmanCode::new(3, 0b_110)).unwrap();
        assert_eq!(writer.cache_len, 7);
        assert_eq!(writer.cache, 0b_1_010_110);

        writer.write(&HuffmanCode::new(1, 0b_0)).unwrap();
        assert_eq!(writer.cache_len, 0);
        assert_eq!(writer.cache, 0);

        writer.close().unwrap();
        assert_eq!(compressed, vec![0b_111_000_00, 0b_1_010_110_0]);
    }

    #[test]
    fn test_write_long_code() {
        let mut compressed = vec![];
        let mut writer = CodeWriter::new(&mut compressed);

        writer
            .write(&HuffmanCode::new(
                32,
                0b_11100011_10001110_11000111_00011100,
            ))
            .unwrap();
        writer.close().unwrap();

        assert_eq!(
            compressed,
            vec![0b_11100011, 0b_10001110, 0b_11000111, 0b_00011100]
        );
    }

    #[test]
    fn test_write_code_sizes_1b() {
        let mut compressed = vec![];
        let mut writer = CodeWriter::new(&mut compressed);
        writer.write(&HuffmanCode::new(4, 0b1100)).unwrap();
        writer.close().unwrap();
        assert_eq!(compressed, vec![0b_1100_0000]);

        let mut compressed = vec![];
        let mut writer = CodeWriter::new(&mut compressed);
        writer.write(&HuffmanCode::new(5, 0b1100)).unwrap();
        writer.close().unwrap();
        assert_eq!(compressed, vec![0b_01100_000]);

        let mut compressed = vec![];
        let mut writer = CodeWriter::new(&mut compressed);
        writer.write(&HuffmanCode::new(6, 0b101100)).unwrap();
        writer.close().unwrap();
        assert_eq!(compressed, vec![0b_101100_00]);

        let mut compressed = vec![];
        let mut writer = CodeWriter::new(&mut compressed);
        writer.write(&HuffmanCode::new(7, 0b1101100)).unwrap();
        writer.close().unwrap();
        assert_eq!(compressed, vec![0b_1101100_0]);

        let mut compressed = vec![];
        let mut writer = CodeWriter::new(&mut compressed);
        writer.write(&HuffmanCode::new(8, 0b11101100)).unwrap();
        writer.close().unwrap();
    }

    #[test]
    fn test_write_code_sizes_over_1b() {
        let mut compressed = vec![];
        let mut writer = CodeWriter::new(&mut compressed);
        writer.write(&HuffmanCode::new(9, 0b11101100_1)).unwrap();
        writer.close().unwrap();
        println!("{:08b}_{:08b}", compressed[0], compressed[1]);
        assert_eq!(compressed, vec![0b_11101100, 0b_1_0000000]);

        let mut compressed = vec![];
        let mut writer = CodeWriter::new(&mut compressed);
        writer.write(&HuffmanCode::new(10, 0b11101100_11)).unwrap();
        writer.close().unwrap();
        println!("{:08b}_{:08b}", compressed[0], compressed[1]);
        assert_eq!(compressed, vec![0b_11101100, 0b_1_1000000]);

        let mut compressed = vec![];
        let mut writer = CodeWriter::new(&mut compressed);
        writer
            .write(&HuffmanCode::new(16, 0b11101100_11001011))
            .unwrap();
        writer.close().unwrap();
        println!("{:08b}_{:08b}", compressed[0], compressed[1]);
        assert_eq!(compressed, vec![0b_11101100, 0b_11001011]);
    }

    #[test]
    fn test_write_code_sizes_over_2b() {
        let mut compressed = vec![];
        let mut writer = CodeWriter::new(&mut compressed);
        writer
            .write(&HuffmanCode::new(17, 0b11101100_11001011_1))
            .unwrap();
        writer.close().unwrap();
        println!("{:08b}_{:08b}", compressed[0], compressed[1]);
        assert_eq!(compressed, vec![0b_11101100, 0b_11001011, 0b_1_0000000]);

        let mut compressed = vec![];
        let mut writer = CodeWriter::new(&mut compressed);
        writer
            .write(&HuffmanCode::new(17, 0b11101100_11001011_0))
            .unwrap();
        writer.close().unwrap();
        println!("{:08b}_{:08b}", compressed[0], compressed[1]);
        assert_eq!(compressed, vec![0b_11101100, 0b_11001011, 0b_0_0000000]);

        let mut compressed = vec![];
        let mut writer = CodeWriter::new(&mut compressed);
        writer
            .write(&HuffmanCode::new(24, 0b11101100_11001011_01111110))
            .unwrap();
        writer.close().unwrap();
        println!("{:08b}_{:08b}", compressed[0], compressed[1]);
        assert_eq!(compressed, vec![0b_11101100, 0b_11001011, 0b_01111110]);
    }

    #[test]
    fn test_write_code_sizes_over_3b() {
        let mut compressed = vec![];
        let mut writer = CodeWriter::new(&mut compressed);
        writer
            .write(&HuffmanCode::new(25, 0b11101100_11001011_01111110_1))
            .unwrap();
        writer.close().unwrap();
        println!("{:08b}_{:08b}", compressed[0], compressed[1]);
        assert_eq!(
            compressed,
            vec![0b_11101100, 0b_11001011, 0b_01111110, 0b_10000000]
        );
    }

    #[test]
    fn test_read_next_bit() {
        let data = vec![0b_11001010, 0b_11100011];
        let map = HashMap::new();
        let mut reader = CodeReader::new(&*data, &map);

        assert_eq!(reader.next_bit().unwrap(), 1);
        assert_eq!(reader.next_bit().unwrap(), 1);
        assert_eq!(reader.next_bit().unwrap(), 0);
        assert_eq!(reader.next_bit().unwrap(), 0);
        assert_eq!(reader.next_bit().unwrap(), 1);
        assert_eq!(reader.next_bit().unwrap(), 0);
        assert_eq!(reader.next_bit().unwrap(), 1);
        assert_eq!(reader.next_bit().unwrap(), 0);

        assert_eq!(reader.next_bit().unwrap(), 1);
        assert_eq!(reader.next_bit().unwrap(), 1);
        assert_eq!(reader.next_bit().unwrap(), 1);
        assert_eq!(reader.next_bit().unwrap(), 0);
        assert_eq!(reader.next_bit().unwrap(), 0);
        assert_eq!(reader.next_bit().unwrap(), 0);
        assert_eq!(reader.next_bit().unwrap(), 1);
        assert_eq!(reader.next_bit().unwrap(), 1);

        assert!(reader.next_bit().is_err());
    }

    #[test]
    fn test_read_unpack_byte() {
        let mut map = HashMap::new();
        map.insert(HuffmanCode::new(3, 0b_000), b'a');
        map.insert(HuffmanCode::new(4, 0b_0010), b'b');
        map.insert(HuffmanCode::new(3, 0b_010), b'c');
        map.insert(HuffmanCode::new(3, 0b_011), b'd');
        map.insert(HuffmanCode::new(1, 0b_1), b'e');

        let data = vec![0b_000_0010_0, 0b_10_011_1_1_0];

        let mut reader = CodeReader::new(&*data, &map);

        assert_eq!(reader.read().unwrap(), b'a');
        assert_eq!(reader.read().unwrap(), b'b');
        assert_eq!(reader.read().unwrap(), b'c');
        assert_eq!(reader.read().unwrap(), b'd');
        assert_eq!(reader.read().unwrap(), b'e');
        assert_eq!(reader.read().unwrap(), b'e');

        assert!(reader.read().is_err());
    }
}
