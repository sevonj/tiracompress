use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::time::Instant;

use byteorder::LittleEndian as LE;
use byteorder::ReadBytesExt;
use clap::Parser;

use tiracompress::HuffmanArchive;
use tiracompress::LzArchive;

const FILE_SIG: [u8; 8] = *b"tira-arc";
const FILE_VER: u32 = 1;

#[repr(u32)]
#[derive(clap::ValueEnum, Clone, Copy, Debug)]
enum CompressAlgo {
    Huffman = 0,
    Lz77 = 1,
    Both = 2,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Create archive
    #[arg(short, long)]
    create: bool,

    /// Extract archive
    #[arg(short = 'x', long)]
    extract: bool,

    /// Input file (required)
    #[arg(short, long)]
    input_filepath: PathBuf,

    /// Output file (optional)
    #[arg(short, long)]
    output_filepath: Option<PathBuf>,

    /// Compression scheme (required for --create)
    #[arg(short, long)]
    algorithm: Option<CompressAlgo>,
}

fn main() {
    let args = Args::parse();

    let t_start = Instant::now();

    if !args.create && !args.extract {
        println!("No operation: Do you want to create an archive or extract it?");
        println!("Bye.");
        return;
    }

    if args.create && args.extract {
        println!("Incompatible operations: You asked to both create an archive, and extract it?");
        println!("Bye.");
        return;
    }

    if args.create {
        let Some(algo) = args.algorithm else {
            println!("Please specify the compression scheme.");
            println!("Bye.");
            return;
        };

        let contents = match std::fs::read(&args.input_filepath) {
            Ok(contents) => contents,
            Err(e) => {
                println!("Input file err: '{e}'");
                println!("Bye.");
                return;
            }
        };

        let out_path = args.output_filepath.as_ref().cloned().unwrap_or_else(|| {
            args.input_filepath.with_extension(
                args.input_filepath
                    .extension()
                    .map(|s| s.to_string_lossy().clone().to_string())
                    .unwrap_or_default()
                    + ".arc",
            )
        });

        if let Err(e) = create_archive(algo, &out_path, contents) {
            println!("Output file err: '{e}'");
            println!("Bye.");
            return;
        }
        println!("Success.");
    } else {
        // These IO error matches could be made much nicer, but who has time for that?

        let mut file = match File::open(&args.input_filepath) {
            Ok(file) => file,
            Err(e) => {
                println!("Input file err: '{e}'");
                println!("Bye.");
                return;
            }
        };

        // 16B Header
        let magic = match file.read_u64::<LE>() {
            Ok(value) => value,
            Err(e) => {
                println!("Input file err: '{e}'");
                println!("Bye.");
                return;
            }
        };
        if magic != u64::from_le_bytes(FILE_SIG) {
            println!("File signature doesn't match. Are you sure it's an archive?");
            return;
        }
        let version = match file.read_u32::<LE>() {
            Ok(value) => value,
            Err(e) => {
                println!("Input file err: '{e}'");
                println!("Bye.");
                return;
            }
        };
        if version != FILE_VER {
            println!("Version mismatch: I'm '{FILE_VER}', got '{FILE_VER}'.");
            return;
        }
        let algo = match file.read_u32::<LE>() {
            Ok(value) => value,
            Err(e) => {
                println!("Input file err: '{e}'");
                println!("Bye.");
                return;
            }
        };
        let algo = match algo {
            0 => CompressAlgo::Huffman,
            1 => CompressAlgo::Lz77,
            2 => CompressAlgo::Both,
            _ => panic!("Unknown algo"),
        };

        let out_path = args.output_filepath.as_ref().cloned().unwrap_or_else(|| {
            args.input_filepath.with_extension(
                args.input_filepath
                    .extension()
                    .map(|s| s.to_string_lossy().clone().to_string())
                    .unwrap_or_default()
                    + ".extracted",
            )
        });

        if let Err(e) = extract_archive(algo, &mut file, &out_path) {
            println!("Output file err: '{e}'");
            println!("Bye.");
            return;
        }
        println!("Success.");
    }

    let t_end = Instant::now();
    println!("Time: {:?}", t_end - t_start);
    println!("Bye.");
}

fn create_archive(
    algo: CompressAlgo,
    out_path: &Path,
    contents: Vec<u8>,
) -> Result<(), std::io::Error> {
    let mut out_file = File::create(out_path)?;
    // Add a 16B header
    out_file.write_all(&FILE_SIG)?;
    out_file.write_all(&FILE_VER.to_le_bytes())?;
    out_file.write_all(&(algo as u32).to_le_bytes())?;

    match algo {
        CompressAlgo::Huffman => {
            let archive = HuffmanArchive::new(contents);
            archive.write(&mut out_file)?;
            Ok(())
        }
        CompressAlgo::Lz77 => {
            let archive = LzArchive::new(contents);
            archive.write(&mut out_file)?;
            Ok(())
        }
        CompressAlgo::Both => {
            let archive = LzArchive::new(contents);
            let mut temp = vec![];
            archive.write(&mut temp)?;
            let archive = HuffmanArchive::new(temp);
            archive.write(&mut out_file)?;
            Ok(())
        }
    }
}

fn extract_archive<R: Read>(
    algo: CompressAlgo,
    reader: &mut R,
    out_path: &Path,
) -> Result<(), std::io::Error> {
    let mut out_file = File::create(out_path)?;

    match algo {
        CompressAlgo::Huffman => {
            let archive = HuffmanArchive::read(reader)?;
            out_file.write_all(archive.data())?;
            Ok(())
        }
        CompressAlgo::Lz77 => {
            let archive = LzArchive::read(reader)?;
            out_file.write_all(archive.data())?;
            Ok(())
        }
        CompressAlgo::Both => {
            let archive = HuffmanArchive::read(reader)?;
            let mut temp = Cursor::new(archive.inner());
            let archive = LzArchive::read(&mut temp)?;
            out_file.write_all(archive.data())?;
            Ok(())
        }
    }
}
