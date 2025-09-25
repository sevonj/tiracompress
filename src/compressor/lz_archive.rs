use std::io::Read;
use std::io::Write;

use super::LzPointer;

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

#[cfg(test)]
mod tests {

    use super::*;
}
