use std::time::Instant;

mod decode;
mod encode;

use encode::compress;
use decode::decompress;

fn main() {
    let now = Instant::now();
    // compress("data/hello.txt", "tmp.bin", 12).unwrap();
    // decompress("tmp.bin", "out.txt").unwrap();

    compress("data/image.bmp", "image.bin", 12).unwrap();
    decompress("image.bin", "image.bmp").unwrap();

    println!("Elapsed: {:.3} (s)", now.elapsed().as_secs_f32());
}
