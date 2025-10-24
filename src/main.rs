use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use std::io::Seek;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;
use std::time::Instant;

use byteorder::LittleEndian as LE;
use byteorder::ReadBytesExt;
use clap::Parser;

use csv::Writer;
use csv::WriterBuilder;
use tiracompress::HuffmanArchive;
use tiracompress::LzArchive;

const FILE_SIG: [u8; 8] = *b"tira-arc";
const FILE_VER: u32 = 1;
const BENCHMARK_PASSES: u32 = 10;

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

    /// Run algorithm benchmark and generate csv.
    #[arg(short, long)]
    benchmark: bool,

    /// Input file (required) Can be a directory if --benchmark.
    #[arg(short, long)]
    input_path: PathBuf,

    /// Output file (optional)
    #[arg(short, long)]
    output_path: Option<PathBuf>,

    /// Compression scheme (required for --create)
    #[arg(short, long)]
    algorithm: Option<CompressAlgo>,
}

fn main() {
    let args = Args::parse();

    let t_start = Instant::now();

    if args.benchmark {
        if args.create {
            println!("Incompatible operations: create can't be used in benchmark");
            println!("Bye.");
            return;
        }
        if args.extract {
            println!("Incompatible operations: extract can't be used in benchmark");
            println!("Bye.");
            return;
        }
        if args.algorithm.is_some() {
            println!("Incompatible operations: algorithm can't be used in benchmark");
            println!("Bye.");
            return;
        }
        if args.output_path.is_some() {
            println!("Incompatible operations: output_path can't be used in benchmark");
            println!("Bye.");
            return;
        }
    } else {
        if !args.create && !args.extract {
            println!("No operation: Do you want to create an archive or extract it?");
            println!("Bye.");
            return;
        }

        if args.create && args.extract {
            println!(
                "Incompatible operations: You asked to both create an archive, and extract it?"
            );
            println!("Bye.");
            return;
        }
    }

    if args.benchmark {
        let record = match benchmark(&args.input_path) {
            Ok(contents) => contents,
            Err(e) => {
                println!("Input file err: '{e}'");
                println!("Bye.");
                return;
            }
        };
    } else if args.create {
        let Some(algo) = args.algorithm else {
            println!("Please specify the compression scheme.");
            println!("Bye.");
            return;
        };

        let contents = match std::fs::read(&args.input_path) {
            Ok(contents) => contents,
            Err(e) => {
                println!("Input file err: '{e}'");
                println!("Bye.");
                return;
            }
        };

        let out_path = args.output_path.as_ref().cloned().unwrap_or_else(|| {
            args.input_path.with_extension(
                args.input_path
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

        let mut file = match File::open(&args.input_path) {
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

        let out_path = args.output_path.as_ref().cloned().unwrap_or_else(|| {
            args.input_path.with_extension(
                args.input_path
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

fn benchmark(input_path: &Path) -> Result<(), std::io::Error> {
    let record_path = input_path.parent().unwrap().join("tiracompress.csv");
    let mut csv = WriterBuilder::new().from_path(record_path)?;
    csv.write_record(&[
        "input_path",
        "og_size",
        "hc_encode",
        "hc_decode",
        "hc_size",
        "lz_encode",
        "lz_decode",
        "lz_size",
    ])?;

    if input_path.is_file() {
        benchmark_file(input_path, &mut csv)?;
    }

    if input_path.is_dir() {
        for entry in std::fs::read_dir(&input_path)? {
            let path = entry?.path();
            let ext = path.extension();
            if ext.is_some_and(|ext| ext == "temp" || ext == "arc") {
                continue;
            }
            if path.is_file() {
                benchmark_file(&path, &mut csv)?;
            }
        }
    }

    csv.flush()?;

    Ok(())
}

fn benchmark_file(input_path: &Path, csv: &mut Writer<File>) -> Result<(), std::io::Error> {
    println!("{input_path:?}");
    let contents = std::fs::read(input_path)?;

    let out_path = input_path.with_extension(
        input_path
            .extension()
            .map(|s| s.to_string_lossy().clone().to_string())
            .unwrap_or_default()
            + ".packed.temp",
    );
    let out_path2 = input_path.with_extension(
        input_path
            .extension()
            .map(|s| s.to_string_lossy().clone().to_string())
            .unwrap_or_default()
            + ".unpacked.temp",
    );

    let og_size = contents.len();
    let mut hc_encode_duration = Duration::ZERO;
    let mut hc_decode_duration = Duration::ZERO;
    let mut hc_size = 0;
    let mut lz_encode_duration = Duration::ZERO;
    let mut lz_decode_duration = Duration::ZERO;
    let mut lz_size = 0;

    for _ in 0..BENCHMARK_PASSES {
        let contents = contents.clone();

        let t_start = Instant::now();
        create_archive(CompressAlgo::Huffman, &out_path, contents)?;
        let duration = Instant::now() - t_start;
        println!("hc encode: {:?}", duration);
        hc_encode_duration += duration;

        let t_start = Instant::now();
        let mut f = File::open(&out_path)?;
        f.seek(std::io::SeekFrom::Current(16))?;
        extract_archive(CompressAlgo::Huffman, &mut f, &out_path2)?;
        let duration = Instant::now() - t_start;
        println!("hc decode: {:?}", duration);
        hc_decode_duration += duration;
    }
    let size = std::fs::metadata(&out_path)?.len();
    println!("packed size relative: '{}'", size as f32 / og_size as f32);
    hc_size = size;

    for _ in 0..BENCHMARK_PASSES {
        let contents = contents.clone();

        let t_start = Instant::now();
        create_archive(CompressAlgo::Lz77, &out_path, contents)?;
        let duration = Instant::now() - t_start;
        println!("lz encode: {:?}", duration);
        lz_encode_duration += duration;

        let t_start = Instant::now();
        let mut f = File::open(&out_path)?;
        f.seek(std::io::SeekFrom::Current(16))?;
        extract_archive(CompressAlgo::Lz77, &mut f, &out_path2)?;
        let duration = Instant::now() - t_start;
        println!("lz decode: {:?}", duration);
        lz_decode_duration += duration;
    }
    let size = std::fs::metadata(&out_path)?.len();
    println!("packed size relative: '{}'", size as f32 / og_size as f32);
    lz_size = size;

    std::fs::remove_file(&out_path)?;
    std::fs::remove_file(&out_path2)?;

    hc_encode_duration /= BENCHMARK_PASSES;
    hc_decode_duration /= BENCHMARK_PASSES;
    lz_encode_duration /= BENCHMARK_PASSES;
    lz_decode_duration /= BENCHMARK_PASSES;

    csv.write_record(&[
        input_path.to_string_lossy().to_string(),
        og_size.to_string(),
        hc_encode_duration.as_secs_f32().to_string(),
        hc_decode_duration.as_secs_f32().to_string(),
        hc_size.to_string(),
        lz_encode_duration.as_secs_f32().to_string(),
        lz_decode_duration.as_secs_f32().to_string(),
        lz_size.to_string(),
    ])?;

    println!("encode avg: {:?}", lz_encode_duration);
    println!("decode avg: {:?}", lz_decode_duration);

    Ok(())
}
