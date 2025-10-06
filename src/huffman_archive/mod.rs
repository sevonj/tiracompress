mod huffman_code;
mod huffman_io;
mod huffman_tree;

use byteorder::LittleEndian as LE;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use std::collections::HashMap;
use std::io::Cursor;
use std::io::Read;
use std::io::Write;

use super::bit_io::BitReader;
use super::bit_io::BitWriter;
use huffman_code::HuffmanCode;
use huffman_io::CodeReader;
use huffman_io::CodeWriter;
use huffman_tree::HuffmanTreeNode;

/// Archive layout:
/// - len_uncompressed: u32
/// - compressed tree (end is byte-aligned)
/// - compressed data
pub struct HuffmanArchive {
    data: Vec<u8>, // Uncompressed
}

impl HuffmanArchive {
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
        let num_bytes = reader.read_u32::<LE>()?;

        let mut bitreader = BitReader::new(&mut *reader);
        let tree = HuffmanTreeNode::read(&mut bitreader)?;
        let codes = tree.into_codes();
        let mut codes_reverse = HashMap::new();
        for (k, v) in codes {
            codes_reverse.insert(v, k);
        }

        let mut data = Vec::with_capacity(num_bytes as usize);
        let mut code_reader = CodeReader::new(reader, &codes_reverse);
        for _ in 0..num_bytes {
            data.push(code_reader.read().unwrap());
        }

        Ok(Self { data })
    }

    /// Pack self
    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        // num_bytes
        writer.write_u32::<LE>(self.data.len() as u32)?;

        let tree = HuffmanTreeNode::from_reader(&mut Cursor::new(&self.data)).unwrap();
        let mut bitwriter = BitWriter::new(&mut *writer);
        tree.write(&mut bitwriter)?;
        bitwriter.close()?;
        let codes = tree.into_codes();

        // compressed_data
        let mut code_writer = CodeWriter::new(writer);
        for byte in &mut Cursor::new(&self.data).bytes() {
            code_writer.write(codes.get(&byte?).unwrap())?;
        }
        code_writer.close()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use std::io::Cursor;

    use super::*;

    // Test compression and decompression without the archive container
    #[test]
    fn test_compress_cycle_text_identical() {
        let data = b"Tomorrow I'll Tomorrow I'll Tomorrow I'll Tomorrow I'll".to_vec();
        let num_bytes = data.len();

        let tree = HuffmanTreeNode::from_reader(&mut Cursor::new(&data)).unwrap();
        let codes = tree.into_codes();

        let mut compressed_data = vec![];
        let mut writer = CodeWriter::new(&mut compressed_data);
        for byte in data.bytes() {
            let byte = byte.unwrap();
            let code = codes.get(&byte).unwrap();
            writer.write(code).unwrap();
        }
        writer.close().unwrap();

        let mut codes_reverse = HashMap::new();
        for (k, v) in codes {
            codes_reverse.insert(v, k);
        }
        let mut uncompressed = vec![];
        let mut reader = CodeReader::new(&*compressed_data, &codes_reverse);
        for _ in 0..num_bytes {
            uncompressed.push(reader.read().unwrap());
        }

        assert_eq!(data, uncompressed);
    }

    // Test compression and decompression without the archive container
    #[test]
    fn test_compress_cycle_data_identical() {
        let data = std::fs::read("samples/salsa.mid").unwrap();
        let num_bytes = data.len();

        let tree = HuffmanTreeNode::from_reader(&mut Cursor::new(&data)).unwrap();
        let codes = tree.into_codes();

        let mut compressed_data = vec![];
        let mut writer = CodeWriter::new(&mut compressed_data);
        for byte in data.bytes() {
            let byte = byte.unwrap();
            let code = codes.get(&byte).unwrap();
            writer.write(code).unwrap();
        }
        writer.close().unwrap();

        let mut codes_reverse = HashMap::new();
        for (k, v) in codes {
            codes_reverse.insert(v, k);
        }
        let mut uncompressed = vec![];
        let mut reader = CodeReader::new(&*compressed_data, &codes_reverse);
        for _ in 0..num_bytes {
            uncompressed.push(reader.read().unwrap());
        }

        assert_eq!(data, uncompressed);
    }

    // Test compression and decompression with the archive container
    #[test]
    fn test_archive_cycle_text_identical() {
        let data = b"Tomorrow I'll Tomorrow I'll Tomorrow I'll Tomorrow I'll".to_vec();

        let arc = HuffmanArchive::new(data.clone());
        let mut compressed = vec![];
        arc.write(&mut Cursor::new(&mut compressed)).unwrap();

        let arc = HuffmanArchive::read(&mut Cursor::new(&mut compressed)).unwrap();
        let uncompressed = arc.inner();

        assert_eq!(data, uncompressed);
    }

    // Test compression and decompression with the archive container
    #[test]
    fn test_archive_cycle_data_identical() {
        let data = std::fs::read("samples/salsa.mid").unwrap();

        let arc = HuffmanArchive::new(data.clone());
        let mut compressed = vec![];
        arc.write(&mut Cursor::new(&mut compressed)).unwrap();

        let arc = HuffmanArchive::read(&mut Cursor::new(&mut compressed)).unwrap();
        let uncompressed = arc.inner();

        assert_eq!(data, uncompressed);
    }
}
