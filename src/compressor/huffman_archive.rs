use crate::compressor::huffman_io::CodeWriter;

use super::HuffmanCode;
use super::HuffmanTreeNode;

use std::collections::HashMap;
use std::io::Read;
use std::io::Seek;

pub struct HuffmanArchive {
    codes: HashMap<u8, HuffmanCode>,
    compressed_data: Vec<u8>,
}

impl HuffmanArchive {
    pub fn new<R: Read + Seek>(reader: &mut R) -> Result<Self, std::io::Error> {
        let start_pos = reader.stream_position()?;
        let tree = HuffmanTreeNode::from_reader(reader)?;
        reader.seek(std::io::SeekFrom::Start(start_pos))?;

        let codes = tree.into_codes();
        let compressed_data = vec![];

        Ok(Self {
            codes,
            compressed_data,
        })
    }

    fn compress<R: Read + Seek>(
        reader: &mut R,
        codes: &HashMap<u8, HuffmanCode>,
    ) -> Result<Vec<u8>, std::io::Error> {
        let mut compressed_data = vec![];
        let mut writer = CodeWriter::new(&mut compressed_data);

        for byte in reader.bytes() {
            writer.write(codes.get(&byte?).unwrap())?;
        }

        Ok(compressed_data)
    }
}

#[cfg(test)]
mod tests {

    use std::io::Cursor;

    use crate::compressor::huffman_io::CodeReader;

    use super::*;

    // Test compression and decompression without the archive container
    #[test]
    fn test_compress_cycle_text() {
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
    fn test_compress_cycle_data() {
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
}
