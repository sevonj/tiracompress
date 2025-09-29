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

    /// Unpack self, space-inefficient whole byte aligned
    pub fn read_byte_align<R: Read>(reader: &mut R) -> Result<Self, std::io::Error> {
        let len = reader.read_u32::<LE>()? as usize;

        let mut data = Vec::with_capacity(len);

        loop {
            if data.len() == len {
                break;
            }

            let is_ptr = reader.read_u8()? > 0;
            if !is_ptr {
                data.push(reader.read_u8()?);
            } else {
                let ptr = LzPointer::from(reader.read_u16::<LE>()?);
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

    /// Pack self, space-inefficient whole byte aligned
    pub fn write_byte_align<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        writer.write_u32::<LE>(self.data.len() as u32)?;

        let mut i = 0;
        while i < self.data.len() {
            if let Some(ptr) = LzPointer::find(&self.data, i)? {
                i += ptr.len();
                writer.write_u8(1_u8)?; // To be packed into 1 bit
                writer.write_u16::<LE>(ptr.into())?;
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

    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_lz_compress_byte_align_text() {
        let input = b"Rep_eat Repeat RepRep";

        let arc = LzArchive::new(input.to_vec());

        let mut output = vec![];
        arc.write_byte_align(&mut output).unwrap();
        let mut result_reader = Cursor::new(output);

        assert_eq!(result_reader.read_u32::<LE>().unwrap(), input.len() as u32);

        for ch in b"Rep_eat " {
            assert_eq!(result_reader.read_u8().unwrap(), 0);
            assert_eq!(result_reader.read_u8().unwrap(), *ch);
        }

        // ptr to "Rep" at the beginning
        let p1 = LzPointer::new(8, 3);
        assert_eq!(result_reader.read_u8().unwrap(), 1);
        assert_eq!(p1, result_reader.read_u16::<LE>().unwrap().into());

        // ptr to "eat Repeat Rep"
        let p2 = LzPointer::new(7, 7);
        assert_eq!(result_reader.read_u8().unwrap(), 1);
        assert_eq!(p2, result_reader.read_u16::<LE>().unwrap().into());

        // ptr to "Rep" at the beginning
        let p3 = LzPointer::new(18, 3);
        assert_eq!(result_reader.read_u8().unwrap(), 1);
        assert_eq!(p3, result_reader.read_u16::<LE>().unwrap().into());
    }

    #[test]
    fn test_lz_compress_byte_align_cycle_text() {
        let input = b"Rep_eat Repeat RepRep";

        let arc = LzArchive::new(input.to_vec());
        let mut compressed = vec![];
        arc.write_byte_align(&mut compressed).unwrap();

        let arc2 = LzArchive::read_byte_align(&mut Cursor::new(compressed)).unwrap();
        let decompressed = arc2.inner();

        assert_eq!(decompressed, input);
    }

    #[test]
    fn test_lz_compress_byte_align_cycle_file() {
        let input = std::fs::read("samples/skrojw.mid").unwrap();

        let arc = LzArchive::new(input.to_vec());
        let mut compressed = vec![];
        arc.write_byte_align(&mut compressed).unwrap();

        let arc2 = LzArchive::read_byte_align(&mut Cursor::new(compressed)).unwrap();
        let decompressed = arc2.inner();

        assert_eq!(decompressed, input);
    }

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
