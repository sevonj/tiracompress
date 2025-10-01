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

            let is_ptr = bitreader.read_bit()? == 1;
            if !is_ptr {
                data.push(bitreader.read_byte()?);
            } else {
                let ptr = LzPointer::from(u16::from_le_bytes([
                    bitreader.read_byte()?,
                    bitreader.read_byte()?,
                ]));
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
            if let Some(ptr) = LzPointer::find(&self.data, i)? {
                i += ptr.len();
                bitwriter.write(1, 1)?;
                let bytes = u16::from(ptr).to_le_bytes();
                bitwriter.write(8, bytes[0])?;
                bitwriter.write(8, bytes[1])?;
            } else {
                bitwriter.write(1, 0)?;
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
