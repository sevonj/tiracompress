use std::cmp::min;
use std::io::Read;
use std::io::Write;

/// Writer for packed data
pub struct BitWriter<W: Write> {
    writer: W,
    /// Cached incomplete byte
    cache: u8,
    /// Number of bits in cache
    cache_len: u8,
}

impl<W: Write> BitWriter<W> {
    pub fn new(writer: W) -> Self {
        BitWriter {
            writer,
            cache: 0,
            cache_len: 0,
        }
    }

    /// Pack a byte
    /// len: number of bits in data
    /// data: bits to pack. Most significant are cut off.u
    pub fn write(&mut self, mut len: u8, mut data: u8) -> Result<(), std::io::Error> {
        data <<= 8 - len;

        while len != 0 {
            let available = 8 - self.cache_len;
            let len_write = min(available, len);
            self.cache <<= len_write % 8;
            let value = data >> (8 - len_write);
            self.cache |= value;
            self.cache_len += len_write;

            if self.cache_len == 8 {
                self.writer.write_all(&[self.cache])?;
                self.cache = 0;
                self.cache_len = 0;
            }

            len -= len_write;
            data <<= len_write;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_whitebox_1b() {
        let mut compressed = vec![];
        let mut writer = BitWriter::new(&mut compressed);

        writer.write(3, 0b_111).unwrap();
        assert_eq!(writer.cache_len, 3);
        assert_eq!(writer.cache, 0b_111);
        writer.write(3, 0b_000).unwrap();
        assert_eq!(writer.cache_len, 6);
        assert_eq!(writer.cache, 0b_111_000);
        writer.write(2, 0b_11).unwrap();
        assert_eq!(writer.cache_len, 0);
        assert_eq!(writer.cache, 0);
        writer.close().unwrap();
        assert_eq!(compressed, vec![0b_111_000_11]);
    }

    #[test]
    fn test_write_whitebox_2b() {
        let mut compressed = vec![];
        let mut writer = BitWriter::new(&mut compressed);

        writer.write(3, 0b_111).unwrap();
        assert_eq!(writer.cache_len, 3);
        assert_eq!(writer.cache, 0b_111);

        writer.write(3, 0b_111).unwrap();
        assert_eq!(writer.cache_len, 6);
        assert_eq!(writer.cache, 0b_111_111);

        writer.write(3, 0b_110).unwrap();
        assert_eq!(writer.cache_len, 1);
        assert_eq!(writer.cache, 0b_0);

        writer.write(3, 0b_000).unwrap();
        assert_eq!(writer.cache_len, 4);
        assert_eq!(writer.cache, 0b_0_000);

        writer.write(3, 0b_000).unwrap();
        assert_eq!(writer.cache_len, 7);
        assert_eq!(writer.cache, 0b_0_000_000);

        writer.write(1, 0b_0).unwrap();
        assert_eq!(writer.cache_len, 0);
        assert_eq!(writer.cache, 0);

        writer.close().unwrap();
        assert_eq!(compressed, vec![0b_111_111_11, 0b_0_000_000_0]);
    }
    #[test]

    fn test_write_whitebox_2b_2() {
        let mut compressed = vec![];
        let mut writer = BitWriter::new(&mut compressed);

        writer.write(3, 0b_111).unwrap();
        assert_eq!(writer.cache_len, 3);
        assert_eq!(writer.cache, 0b_111);

        writer.write(3, 0b_000).unwrap();
        assert_eq!(writer.cache_len, 6);
        assert_eq!(writer.cache, 0b_111_000);

        writer.write(3, 0b_001).unwrap();
        assert_eq!(writer.cache_len, 1);
        assert_eq!(writer.cache, 0b_1);

        writer.write(3, 0b_010).unwrap();
        assert_eq!(writer.cache_len, 4);
        assert_eq!(writer.cache, 0b_1_010);

        writer.write(3, 0b_110).unwrap();
        assert_eq!(writer.cache_len, 7);
        assert_eq!(writer.cache, 0b_1_010_110);

        writer.write(1, 0b_0).unwrap();
        assert_eq!(writer.cache_len, 0);
        assert_eq!(writer.cache, 0);

        writer.close().unwrap();
        assert_eq!(compressed, vec![0b_111_000_00, 0b_1_010_110_0]);
    }
}
