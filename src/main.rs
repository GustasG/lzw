use std::path::Path;
use std::time::Instant;

mod args;
mod lzw;

use args::{Arguments, Mode};
use clap::Parser;
use lzw::{compress_file, decompress_file};

fn run_compression(input_path: &Path, output_path: &Path, length: u8) {
    let now = Instant::now();

    if let Err(e) = compress_file(input_path, output_path, length) {
        eprintln!("Error failed to compress: {}", e);
    } else {
        let input_size = input_path.metadata().unwrap().len();
        let output_size = output_path.metadata().unwrap().len();
        let compression_ratio = input_size as f32 / output_size as f32;
        let duration = now.elapsed();

        println!("-------------------------------------");
        println!("Compression finished");
        println!("Input file size: {} bytes", input_size);
        println!("Output file size: {} bytes", output_size);
        println!(
            "Compression ratio: {:.3} ({:.2} %)",
            compression_ratio,
            compression_ratio * 100.0
        );
        println!("Elapsed: {:.3} (s)", duration.as_secs_f32());
    }
}

fn run_decompression(input_path: &Path, output_path: &Path) {
    let now = Instant::now();

    if let Err(e) = decompress_file(input_path, output_path) {
        eprintln!("Error failed to decompress: {}", e);
    } else {
        let input_size = input_path.metadata().unwrap().len();
        let output_size = output_path.metadata().unwrap().len();
        let duration = now.elapsed();

        println!("-------------------------------------");
        println!("Decompression finished");
        println!("Input file size: {} bytes", input_size);
        println!("Output file size: {} bytes", output_size);
        println!("Elapsed: {:.3} (s)", duration.as_secs_f32());
    }
}

fn main() {
    let args = Arguments::parse();

    match args.mode {
        Mode::Compress => run_compression(&args.input_file, &args.output_file, args.length),
        Mode::Decompress => run_decompression(&args.input_file, &args.output_file),
    }
}
