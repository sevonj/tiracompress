use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use tiracompress::HuffmanArchive;

fn main() {
    let path = PathBuf::from("samples/powerpark.wav");
    let f = File::open(&path).unwrap();
    let mut reader = BufReader::new(&f);

    let archive = HuffmanArchive::compress(&mut reader);
}
