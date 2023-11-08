use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

use bitstream_io::{BigEndian, BitRead, BitReader, Endianness};

fn decompress_stream<R: Read, W: Write, E: Endianness>(
    reader: &mut BitReader<R, E>,
    writer: &mut W,
) -> Result<(), std::io::Error> {
    let max_code_size = reader.read::<u8>(4)?;

    if max_code_size < 8 || max_code_size > 16 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "invalid max_code_size",
        ));
    }

    let max_entries = 2_usize.pow(max_code_size as u32);
    let mut table: HashMap<u16, Vec<u8>> = (0..256).map(|i| (i as u16, vec![i as u8])).collect();

    let mut prev_code = match reader.read::<u16>(max_code_size as u32) {
        Err(e) => match e.kind() {
            std::io::ErrorKind::UnexpectedEof => return Ok(()),
            _ => return Err(e),
        },
        Ok(code) => code,
    };

    writer.write(&table[&prev_code])?;

    loop {
        match reader.read::<u16>(max_code_size as u32) {
            Err(e) => match e.kind() {
                std::io::ErrorKind::UnexpectedEof => break,
                _ => return Err(e),
            },
            Ok(code) => {
                let new_entry = match table.get(&code) {
                    Some(entry) => {
                        writer.write(&entry)?;

                        let mut prev = table[&prev_code].clone();
                        prev.push(entry[0]);
                        prev
                    }
                    None => {
                        let mut v = table[&prev_code].clone();
                        v.push(v[0]);

                        writer.write(&v)?;

                        v
                    }
                };

                if table.len() == max_entries && max_entries > 256 {
                    table = (0..256).map(|i| (i as u16, vec![i as u8])).collect();
                }

                if table.len() < max_entries {
                    table.insert(table.len() as u16, new_entry);
                }

                prev_code = code;
            }
        }
    }

    Ok(())
}

pub fn decompress<P: AsRef<Path>>(input_path: P, output_path: P) -> Result<(), std::io::Error> {
    let fin = File::open(input_path)?;
    let buf_reader = BufReader::with_capacity(32 * 1024, fin);
    let mut reader = BitReader::endian(buf_reader, BigEndian);

    let fout = File::create(output_path)?;
    let mut writer = BufWriter::with_capacity(32 * 1024, fout);

    decompress_stream(&mut reader, &mut writer)
}
