use super::HuffmanCode;
use super::HuffmanTreeNode;

use std::collections::HashMap;
use std::io::Read;

pub struct HuffmanArchive {
    codes: HashMap<u8, HuffmanCode>,
    compressed_data: Vec<u8>,
}

impl HuffmanArchive {
    pub fn compress<R: Read>(reader: &mut R) -> Result<Self, std::io::Error> {
        let tree = HuffmanTreeNode::build_tree(reader)?;
        let codes = tree.into_codes();
        let compressed_data = vec![];

        Ok(Self {
            codes,
            compressed_data,
        })
    }
}
