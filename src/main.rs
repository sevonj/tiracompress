use std::path::PathBuf;

use tiracompress::HuffmanArchive;

fn main() {
    let path = PathBuf::from("samples/powerpark.wav");

    let data = std::fs::read(path).unwrap();

    let archive = HuffmanArchive::new(data);
}
