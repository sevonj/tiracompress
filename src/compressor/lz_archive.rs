use std::io::Read;
use std::io::Write;

pub struct LzArchive {
    data: Vec<u8>, // Uncompressed
}

impl LzArchive {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn inner(self) -> Vec<u8> {
        self.data
    }

    /// Unpack self
    pub fn read<R: Read>(_reader: &mut R) -> Result<Self, std::io::Error> {
        todo!()
    }

    /// Pack self
    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct LzPointer {
    off: isize, // Relative to current read pos
    len: usize, // Number of bytes
}

impl LzPointer {
    pub const fn off(&self) -> isize {
        self.off
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub fn new(off: isize, len: usize) -> Self {
        Self { off, len }
    }

    pub fn find(
        input: &[u8],
        input_pos: usize,
        output: &[u8],
    ) -> Result<Option<Self>, std::io::Error> {
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
                println!("match: {i}, len: {best_len}, off: {best_off}");
            }
        }

        if best_len > 2 {
            return Ok(Some(Self {
                off: best_off as isize - input_pos as isize,
                len: best_len,
            }));
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_pointer_find_match() {
        let input = b"Rep_eat Repeat Repeat";
        let output = b"Repeat ".to_vec();

        // pos 8 is the second 'R'
        let result = LzPointer::find(input, 8, &output).unwrap().unwrap();

        assert_eq!(result, LzPointer::new(-8, 3));
    }
}
