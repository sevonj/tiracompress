use crate::compressor::huffman_io::CodeWriter;

use super::HuffmanCode;
use super::HuffmanTreeNode;

use std::collections::HashMap;
use std::io::BufWriter;
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
