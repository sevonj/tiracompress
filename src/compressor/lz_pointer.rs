use std::io::Read;
use std::io::Write;

use super::BitReader;
use super::BitWriter;

/// Pointer type for repeated data.
/// Packed size (in bits): NUM_BITS_LEN, if len == 0, else NUM_BITS_LEN + NUM_BITS_OFF
#[derive(Debug, PartialEq, Eq)]
pub struct LzPointer {
    /// Number of repeat bytes
    /// 4 bits => max 15B
    len: usize,
    /// How far is it from current position
    /// 12 bits => max 4095B
    off: usize,
}

impl LzPointer {
    /// How many bits to use for length?
    pub const BITS_IN_LEN: usize = 4;
    /// How many bits to use for offset?
    pub const BITS_IN_OFF: usize = 12;

    pub const MAX_LEN: usize = 2_usize.pow(Self::BITS_IN_LEN as u32) - 1;
    pub const MAX_OFF: usize = 2_usize.pow(Self::BITS_IN_OFF as u32) - 1;

    pub const fn off(&self) -> usize {
        self.off
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn new(off: usize, len: usize) -> Self {
        debug_assert!(off <= Self::MAX_OFF);
        debug_assert!(len <= Self::MAX_LEN);

        Self { off, len }
    }

    /// Not a pointer, literal data
    pub const fn literal() -> Self {
        Self { off: 0, len: 0 }
    }

    /// Am I not a pointer?
    pub const fn is_literal(&self) -> bool {
        self.len == 0
    }

    /// Deserialize.
    pub fn read<R: Read>(bitreader: &mut BitReader<R>) -> Result<Self, std::io::Error> {
        let mut len = 0;
        for _ in 0..Self::BITS_IN_LEN {
            len <<= 1;
            len += bitreader.read_bit()? as usize;
        }

        // Not a pointer, we can skip offset bytes
        if len == 0 {
            return Ok(Self::literal());
        }

        let mut off = 0;
        for _ in 0..Self::BITS_IN_OFF {
            off <<= 1;
            off += bitreader.read_bit()? as usize;
        }

        Ok(Self { len, off })
    }

    /// Serialize
    pub fn write<W: Write>(&self, bitwriter: &mut BitWriter<W>) -> Result<(), std::io::Error> {
        for i in (0..Self::BITS_IN_LEN).rev() {
            bitwriter.write(1, (self.len >> i) as u8)?;
        }

        // Not a pointer, we can skip offset bytes
        if self.len == 0 {
            return Ok(());
        }

        for i in (0..Self::BITS_IN_OFF).rev() {
            bitwriter.write(1, (self.off >> i) as u8)?;
        }

        Ok(())
    }

    /// Find longest match
    pub fn find(input: &[u8], input_pos: usize) -> Result<Self, std::io::Error> {
        let input_start = input_pos.saturating_sub(Self::MAX_OFF);

        let mut best_len = 0;
        let mut best_off = 0;

        for off in input_start..input_pos {
            // When either max len or end of input buffer is reached.
            let too_far = std::cmp::min(input_pos + Self::MAX_LEN, input.len());

            for (i, byte) in input[off..].iter().enumerate() {
                let off_input_char = input_pos + i;
                if i > best_len {
                    best_off = off;
                    best_len = i;
                }
                if off_input_char == too_far {
                    break;
                }
                if *byte != input[off_input_char] {
                    break;
                }
            }
        }

        if best_len < 3 {
            return Ok(Self::literal());
        }

        debug_assert!(input_pos - best_off <= Self::MAX_OFF);
        debug_assert!(best_len <= Self::MAX_LEN);

        Ok(Self {
            off: input_pos - best_off,
            len: best_len,
        })
    }
}

/*
impl From<u16> for LzPointer {
    fn from(value: u16) -> Self {
        let off = value & 0xfff;
        let len = value >> 12;

        Self {
            off: off as usize,
            len: len as usize,
        }
    }
}

impl From<LzPointer> for u16 {
    fn from(value: LzPointer) -> Self {
        let off = value.off as u16;
        let len = (value.len << 12) as u16;
        off + len
    }
}
// */

#[cfg(test)]
mod tests {

    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_pointer_find_match() {
        let input = b"Rep_eat Repeat Repeat";

        // pos 8 is the second 'R'
        let result = LzPointer::find(input, 8).unwrap();

        assert_eq!(result, LzPointer::new(8, 3));
    }

    // Match ends after input pos
    #[test]
    fn test_pointer_find_match_exceed() {
        let input = b"abcd_abcd_abcd-xyzxyzxyzx";

        // pos 7 is the second 'c'
        let result = LzPointer::find(input, 7).unwrap();

        assert_eq!(result, LzPointer::new(5, 7));
    }

    // Tests match minimum limit. TODO: actually think about the limit
    #[test]
    fn test_pointer_find_match_3b() {
        let input = b"abcdefffghifffjklmn";

        // pos 11 is the start of the second "fff"
        let result = LzPointer::find(input, 11).unwrap();

        assert_eq!(result, LzPointer::new(6, 3));
    }

    // Tests match minimum limit. TODO: actually think about the limit
    #[test]
    fn test_pointer_find_match_2b() {
        let input = b"abcdeffxghiffyjklmn";

        // pos 11 is the start of the second "ff"
        let result = LzPointer::find(input, 11).unwrap();

        assert!(result.is_literal());
    }

    /*
    #[test]
    fn test_pointer_from_u16() {
        assert_eq!(LzPointer::from(0x1234), LzPointer::new(0x234, 0x1));
        assert_eq!(LzPointer::from(0xffff), LzPointer::new(0xfff, 0xf));
        assert_eq!(LzPointer::from(0x0ff0), LzPointer::new(0xff0, 0x0));
    }

    #[test]
    fn test_pointer_to_u16() {
        assert_eq!(u16::from(LzPointer::new(0x512, 0x3)), 0x3512);
        assert_eq!(u16::from(LzPointer::new(0x00f, 0xe)), 0xe00f);
        assert_eq!(u16::from(LzPointer::new(0xff0, 0x0)), 0x0ff0);
    }
    // */

    #[test]
    fn test_pointer_read_none() {
        let reader = Cursor::new([0b_00001111_u8, 0b_00001111_u8]);
        let mut bitreader = BitReader::new(reader);

        assert_eq!(
            LzPointer::read(&mut bitreader).unwrap(),
            LzPointer::new(0, 0)
        );
    }

    #[test]
    fn test_pointer_read_some() {
        let reader = Cursor::new([0b_01001111_u8, 0b_00001111_u8]);
        let mut bitreader = BitReader::new(reader);

        assert_eq!(
            LzPointer::read(&mut bitreader).unwrap(),
            LzPointer::new(0b_111100001111, 0b_0100)
        );
    }

    #[test]
    fn test_pointer_write_none() {
        let mut data = vec![0b_00001111_u8, 0b_00001111_u8];
        let mut bitwriter = BitWriter::new(&mut data);
        LzPointer::new(0, 0).write(&mut bitwriter).unwrap();
        bitwriter.write(8, 0xff).unwrap();
        bitwriter.close().unwrap();
        assert_eq!(
            *&data,
            [0b_00001111_u8, 0b_00001111_u8, 0b_0000_1111, 0b_1111_0000]
        );
    }

    #[test]
    fn test_pointer_read_write_cycle() {
        let mut data = vec![];
        let mut bitwriter = BitWriter::new(&mut data);

        let ptr_a = LzPointer::new(0b_0101_0001_1000, 0b_0101);
        let ptr_b = LzPointer::new(0b_0001_0000_0110, 0b_0001);
        let ptr_c = LzPointer::new(0b_0000_0000_0000, 0b_1101);
        let ptr_d = LzPointer::new(0b_1111_1111_1110, 0b_1110);
        let ptr_e = LzPointer::new(0b_1111_0110_0000, 0b_1111);
        let ptr_f = LzPointer::new(0b_0110_0011_0000, 0b_0110);
        let ptr_g = LzPointer::new(0b_0111_0000_0111, 0b_0111);

        ptr_a.write(&mut bitwriter).unwrap();
        ptr_b.write(&mut bitwriter).unwrap();
        ptr_c.write(&mut bitwriter).unwrap();
        ptr_d.write(&mut bitwriter).unwrap();
        ptr_e.write(&mut bitwriter).unwrap();
        ptr_f.write(&mut bitwriter).unwrap();
        ptr_g.write(&mut bitwriter).unwrap();
        bitwriter.close().unwrap();

        let mut cursor = Cursor::new(&data);
        let mut bitreader = BitReader::new(&mut cursor);

        assert_eq!(LzPointer::read(&mut bitreader).unwrap(), ptr_a);
        assert_eq!(LzPointer::read(&mut bitreader).unwrap(), ptr_b);
        assert_eq!(LzPointer::read(&mut bitreader).unwrap(), ptr_c);
        assert_eq!(LzPointer::read(&mut bitreader).unwrap(), ptr_d);
        assert_eq!(LzPointer::read(&mut bitreader).unwrap(), ptr_e);
        assert_eq!(LzPointer::read(&mut bitreader).unwrap(), ptr_f);
        assert_eq!(LzPointer::read(&mut bitreader).unwrap(), ptr_g);
    }
}
