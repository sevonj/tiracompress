use byteorder::LittleEndian as LE;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use std::io::Read;
use std::io::Write;

use super::BitReader;
use super::BitWriter;
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
    pub fn read<R: Read>(reader: &mut R) -> Result<Self, std::io::Error> {
        let len = reader.read_u32::<LE>()? as usize;

        let mut bitreader = BitReader::new(reader);

        let mut data = Vec::with_capacity(len);

        loop {
            if data.len() == len {
                break;
            }

            let ptr = LzPointer::read(&mut bitreader)?;

            if ptr.is_literal() {
                data.push(bitreader.read_byte()?);
            } else {
                let start = data.len() - ptr.off();
                let end = start + ptr.len();

                for i in start..end {
                    data.push(data[i]);
                }
            }
        }

        Ok(Self { data })
    }

    /// Pack self
    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        writer.write_u32::<LE>(self.data.len() as u32)?;

        let mut bitwriter = BitWriter::new(writer);

        let mut i = 0;
        while i < self.data.len() {
            let ptr = LzPointer::find(&self.data, i)?;
            ptr.write(&mut bitwriter)?;
            i += ptr.len();

            if ptr.is_literal() {
                bitwriter.write(8, self.data[i])?;
                i += 1;
            }
        }
        bitwriter.close()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_lz_compress_packed_cycle_text() {
        let input = b"Crud and sludge";

        let arc = LzArchive::new(input.to_vec());
        let mut compressed = vec![];
        arc.write(&mut compressed).unwrap();

        let arc2 = LzArchive::read(&mut Cursor::new(compressed)).unwrap();
        let decompressed = arc2.inner();

        assert_eq!(decompressed, input);
    }

    #[test]
    fn test_lz_compress_packed_cycle_file() {
        let input = std::fs::read("samples/skrojw.mid").unwrap();

        let arc = LzArchive::new(input.to_vec());
        let mut compressed = vec![];
        arc.write(&mut compressed).unwrap();

        let arc2 = LzArchive::read(&mut Cursor::new(compressed)).unwrap();
        let decompressed = arc2.inner();

        assert_eq!(decompressed, input);
    }
}
