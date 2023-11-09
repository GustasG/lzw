use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

use bitstream_io::{BigEndian, BitWrite, BitWriter, Endianness};

fn compress_stream<R: Read, W: Write, E: Endianness>(
    reader: &mut R,
    writer: &mut BitWriter<W, E>,
    max_code_size: u8,
) -> Result<(), std::io::Error> {
    if !(8..16).contains(&max_code_size) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!(
                "max_code_size must be between 8 and 16, got {}",
                max_code_size
            ),
        ));
    }

    let max_entries = 2_usize.pow(max_code_size as u32);
    let mut table: HashMap<Vec<u8>, u16> = (0..256).map(|i| (vec![i as u8], i as u16)).collect();
    let mut word = Vec::new();

    writer.write(8, max_code_size)?;

    for byte in reader.bytes() {
        let byte = byte?;
        word.push(byte);

        if !table.contains_key(&word) {
            word.pop();
            writer.write(max_code_size as u32, table[&word])?;

            if table.len() == max_entries && max_entries > 256 {
                table = (0..256).map(|i| (vec![i as u8], i as u16)).collect();
            }

            if table.len() < max_entries {
                word.push(byte);
                table.insert(std::mem::take(&mut word), table.len() as u16);
            }

            word = vec![byte];
        }
    }

    if !word.is_empty() {
        writer.write(max_code_size as u32, table[&word])?;
    }

    writer.byte_align()?;

    Ok(())
}

pub fn compress<P: AsRef<Path>>(
    input_path: P,
    output_path: P,
    max_code_size: u8,
) -> Result<(), std::io::Error> {
    let fin = File::open(input_path)?;
    let mut reader = BufReader::with_capacity(32 * 1024, fin);

    let fout = File::create(output_path)?;
    let buf_writer = BufWriter::with_capacity(32 * 1024, fout);
    let mut writer = BitWriter::endian(buf_writer, BigEndian);

    compress_stream(&mut reader, &mut writer, max_code_size)
}
