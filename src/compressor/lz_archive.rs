use byteorder::LittleEndian as LE;
use byteorder::WriteBytesExt;
use std::io::Read;
use std::io::Write;

use super::LzPointer;

//const LEN_SEARCH_WINDOW: usize = 8;

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
        let mut i = 0;
        while i < self.data.len() {
            if let Some(ptr) = LzPointer::find(&self.data, i)? {
                writer.write_u8(1_u8)?; // To be packed into 1 bit
                writer.write_u8(ptr.off() as u8)?; // To be packed into n bits
                writer.write_u8(ptr.len() as u8)?; // To be packed into m bits
                i += ptr.len();
            } else {
                writer.write_u8(0_u8)?; // To be packed into 1 bit
                writer.write_u8(self.data[i])?;
                i += 1;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_lz_compress_whitebox() {
        let input = b"Rep_eat Repeat RepRep";

        let arc = LzArchive::new(input.to_vec());

        let mut output = vec![];
        arc.write(&mut output).unwrap();

        let rc = b'R';
        let e = b'e';
        let p = b'p';
        let underscore = b'_';
        let a = b'a';
        let space = b' ';
        let t = b't';

        let expected = vec![
            0, rc, 0, e, 0, p, 0, underscore, 0, e, 0, a, 0, t, 0, space, //
            1, 8, 3, // ptr to "Rep"
            1, 7, 7, // ptr to "eat Repeat Rep"
            1, 18, 3, // ptr to "Rep" at the beginning
        ];

        assert_eq!(output, expected)
    }
}
