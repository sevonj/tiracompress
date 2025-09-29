/// Pointer type for repeated data. Packs into 2B.
/// It's unclear if 4b+12b=16b is optimal for arbitrary binary data,
/// but it's what the sources used.
#[derive(Debug, PartialEq, Eq)]
pub struct LzPointer {
    /// How far is it from current position
    /// 12 bits => max 4096B
    off: usize,
    /// Number of repeat bytes
    /// 4 bits => max 16B
    len: usize,
}

impl LzPointer {
    const MAX_OFF: usize = 0x0fff;

    const MAX_LEN: usize = 0x000f;

    pub const fn off(&self) -> usize {
        self.off
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub fn new(off: usize, len: usize) -> Self {
        debug_assert!(off <= Self::MAX_OFF);
        debug_assert!(len <= Self::MAX_LEN);

        Self { off, len }
    }

    pub fn find(input: &[u8], input_pos: usize) -> Result<Option<Self>, std::io::Error> {
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
            return Ok(None);
        }

        debug_assert!(input_pos - best_off <= Self::MAX_OFF);
        debug_assert!(best_len <= Self::MAX_LEN);

        Ok(Some(Self {
            off: input_pos - best_off,
            len: best_len,
        }))
    }
}

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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_pointer_find_match() {
        let input = b"Rep_eat Repeat Repeat";

        // pos 8 is the second 'R'
        let result = LzPointer::find(input, 8).unwrap().unwrap();

        assert_eq!(result, LzPointer::new(8, 3));
    }

    // Match ends after input pos
    #[test]
    fn test_pointer_find_match_exceed() {
        let input = b"abcd_abcd_abcd-xyzxyzxyzx";

        // pos 7 is the second 'c'
        let result = LzPointer::find(input, 7).unwrap().unwrap();

        assert_eq!(result, LzPointer::new(5, 7));
    }

    // Tests match minimum limit. TODO: actually think about the limit
    #[test]
    fn test_pointer_find_match_3b() {
        let input = b"abcdefffghifffjklmn";

        // pos 11 is the start of the second "fff"
        let result = LzPointer::find(input, 11).unwrap().unwrap();

        assert_eq!(result, LzPointer::new(6, 3));
    }

    // Tests match minimum limit. TODO: actually think about the limit
    #[test]
    fn test_pointer_find_match_2b() {
        let input = b"abcdeffxghiffyjklmn";

        // pos 11 is the start of the second "ff"
        let result = LzPointer::find(input, 11).unwrap();

        assert_eq!(result, None);
    }

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
}
