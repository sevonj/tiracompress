#[derive(Debug, PartialEq, Eq)]
pub struct LzPointer {
    off: usize, // How far is it from input pos
    len: usize, // Number of bytes
}

impl LzPointer {
    pub const fn off(&self) -> usize {
        self.off
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub fn new(off: usize, len: usize) -> Self {
        Self { off, len }
    }

    pub fn find(input: &[u8], input_pos: usize) -> Result<Option<Self>, std::io::Error> {
        let mut best_len = 0;
        let mut best_off = 0;

        for off in 0..input_pos {
            for (i, byte) in input[off..].iter().enumerate() {
                let off_input_char = input_pos + i;
                if i > best_len {
                    best_off = off;
                    best_len = i;
                }
                if off_input_char == input.len() {
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

        Ok(Some(Self {
            off: input_pos - best_off,
            len: best_len,
        }))
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
}
