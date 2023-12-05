use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, ErrorKind, Write};
use std::path::Path;

use bitstream_io::{BigEndian, BitRead, BitReader};

fn decompress_with_infinite_table<R: BitRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
) -> Result<(), std::io::Error> {
    let mut width = 8;
    let mut table = (0..=255)
        .map(|i| (i, vec![i as u8]))
        .collect::<HashMap<u64, Vec<u8>>>();

    let mut prev_code = match reader.read::<u64>(width) {
        Ok(code) => code,
        Err(e) => match e.kind() {
            ErrorKind::UnexpectedEof => return Ok(()),
            _ => return Err(e),
        },
    };

    writer.write_all(&table[&prev_code])?;

    loop {
        if table.len() >= (1 << width) - 1 {
            width += 1;
        }

        match reader.read::<u64>(width) {
            Err(e) => match e.kind() {
                ErrorKind::UnexpectedEof => break,
                _ => return Err(e),
            },
            Ok(code) => {
                let new_entry = match table.get(&code) {
                    Some(entry) => {
                        writer.write_all(entry)?;

                        let mut prev = table[&prev_code].clone();
                        prev.push(entry[0]);
                        prev
                    }
                    None => {
                        let mut v = table[&prev_code].clone();
                        v.push(v[0]);

                        writer.write_all(&v)?;

                        v
                    }
                };

                table.insert(table.len() as u64, new_entry);
                prev_code = code;
            }
        }
    }

    Ok(())
}

fn decompress_with_fixed_table<R: BitRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    table_size: u8,
) -> Result<(), std::io::Error> {
    let max_entries = 2_usize.pow(table_size as u32);
    let mut width = 8;
    let mut table = (0..=255)
        .map(|i| (i, vec![i as u8]))
        .collect::<HashMap<u64, Vec<u8>>>();

    let mut prev_code = match reader.read::<u64>(width) {
        Ok(code) => code,
        Err(e) => match e.kind() {
            ErrorKind::UnexpectedEof => return Ok(()),
            _ => return Err(e),
        },
    };

    writer.write_all(&table[&prev_code])?;

    loop {
        if table.len() >= (1 << width) - 1 {
            width += 1;
        }

        match reader.read::<u64>(width) {
            Err(e) => match e.kind() {
                ErrorKind::UnexpectedEof => break,
                _ => return Err(e),
            },
            Ok(code) => {
                let new_entry = match table.get(&code) {
                    Some(entry) => {
                        writer.write_all(entry)?;

                        let mut prev = table[&prev_code].clone();
                        prev.push(entry[0]);
                        prev
                    }
                    None => {
                        let mut v = table[&prev_code].clone();
                        v.push(v[0]);

                        writer.write_all(&v)?;

                        v
                    }
                };

                if table.len() < max_entries {
                    table.insert(table.len() as u64, new_entry);
                }

                prev_code = code;
            }
        }
    }

    Ok(())
}

fn decompress_with_resizable_table<R: BitRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    table_size: u8,
) -> Result<(), std::io::Error> {
    let max_entries = 2_usize.pow(table_size as u32);
    let mut width = 8;
    let mut table = (0..=255)
        .map(|i| (i, vec![i as u8]))
        .collect::<HashMap<u64, Vec<u8>>>();

    let mut prev_code = match reader.read::<u64>(width) {
        Ok(code) => code,
        Err(e) => match e.kind() {
            ErrorKind::UnexpectedEof => return Ok(()),
            _ => return Err(e),
        },
    };

    writer.write_all(&table[&prev_code])?;

    loop {
        if table.len() >= (1 << width) - 1 {
            width += 1;
        }

        match reader.read::<u64>(width) {
            Err(e) => match e.kind() {
                ErrorKind::UnexpectedEof => break,
                _ => return Err(e),
            },
            Ok(code) => {
                let new_entry = match table.get(&code) {
                    Some(entry) => {
                        writer.write_all(entry)?;

                        let mut prev = table[&prev_code].clone();
                        prev.push(entry[0]);
                        prev
                    }
                    None => {
                        let mut v = table[&prev_code].clone();
                        v.push(v[0]);

                        writer.write_all(&v)?;

                        v
                    }
                };

                if table.len() == max_entries && max_entries > 256 {
                    table = (0..=255).map(|i| (i, vec![i as u8])).collect();
                }

                if table.len() < max_entries {
                    table.insert(table.len() as u64, new_entry);
                }

                prev_code = code;
            }
        }
    }

    Ok(())
}

pub fn decompress_file<P: AsRef<Path>>(
    input_path: P,
    output_path: P,
) -> Result<(), std::io::Error> {
    let fin = File::open(&input_path)?;
    let reader = BufReader::with_capacity(32 * 1024, fin);
    let mut reader = BitReader::endian(reader, BigEndian);

    let fout = File::create(&output_path)?;
    let mut writer = BufWriter::with_capacity(32 * 1024, fout);

    let table_size = reader.read::<u8>(8)?;

    if table_size == 0 {
        decompress_with_infinite_table(&mut reader, &mut writer)
    } else {
        if reader.read_bit()? {
            decompress_with_resizable_table(&mut reader, &mut writer, table_size)
        } else {
            decompress_with_fixed_table(&mut reader, &mut writer, table_size)
        }
    }
}
