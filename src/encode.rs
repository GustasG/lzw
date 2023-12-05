use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, ErrorKind, Read};
use std::path::Path;

use bitstream_io::{BigEndian, BitWrite, BitWriter};

fn validate_table_size(table_size: u8) -> Result<(), std::io::Error> {
    if !(8..=16).contains(&table_size) {
        return Err(std::io::Error::new(
            ErrorKind::InvalidInput,
            format!("size must be between 8 and 16, got {}", table_size),
        ));
    }

    Ok(())
}

fn compress_with_infinite_table<R: Read, W: BitWrite>(
    reader: &mut R,
    writer: &mut W,
) -> Result<(), std::io::Error> {
    let mut width = 8;
    let mut word = Vec::new();
    let mut table = (0..=255)
        .map(|i| (vec![i as u8], i))
        .collect::<HashMap<Vec<u8>, u64>>();

    writer.write(8, 0)?; // table size - hint for decoder to use infinite table decoding

    for byte in reader.bytes() {
        let byte = byte?;
        word.push(byte);

        if !table.contains_key(&word) {
            word.pop();
            writer.write(width, table[&word])?;

            word.push(byte);
            table.insert(std::mem::take(&mut word), table.len() as u64);

            if table.len() >= 1 << width {
                width += 1;
            }

            word = vec![byte];
        }
    }

    if !word.is_empty() {
        writer.write(width, table[&word])?;
    }

    writer.byte_align()?;

    Ok(())
}

fn compress_with_fixed_table<R: Read, W: BitWrite>(
    reader: &mut R,
    writer: &mut W,
    table_size: u8,
) -> Result<(), std::io::Error> {
    validate_table_size(table_size)?;

    let max_entries = 2_usize.pow(table_size as u32);
    let mut width = 8;
    let mut word = Vec::new();
    let mut table = (0..=255)
        .map(|i| (vec![i as u8], i))
        .collect::<HashMap<Vec<u8>, u64>>();

    writer.write(8, table_size)?; // table size
    writer.write_bit(false)?; // hint for decoder to use fixed table decoding

    for byte in reader.bytes() {
        let byte = byte?;
        word.push(byte);

        if !table.contains_key(&word) {
            word.pop();

            writer.write(width, table[&word])?;

            if table.len() < max_entries {
                word.push(byte);
                table.insert(std::mem::take(&mut word), table.len() as u64);

                if table.len() >= 1 << width {
                    width += 1;
                }
            }

            word = vec![byte];
        }
    }

    if !word.is_empty() {
        writer.write(width, table[&word])?;
    }

    writer.byte_align()?;

    Ok(())
}

fn compress_with_resizable_table<R: Read, W: BitWrite>(
    reader: &mut R,
    writer: &mut W,
    table_size: u8,
) -> Result<(), std::io::Error> {
    validate_table_size(table_size)?;

    let max_entries = 2_usize.pow(table_size as u32);
    let mut width = 8;
    let mut table = (0..=255)
        .map(|i| (vec![i as u8], i))
        .collect::<HashMap<Vec<u8>, u64>>();

    let mut word = Vec::new();

    writer.write(8, table_size)?; // table size
    writer.write_bit(true)?; // hint for decoder to use resizable table decoding

    for byte in reader.bytes() {
        let byte = byte?;
        word.push(byte);

        if !table.contains_key(&word) {
            word.pop();

            writer.write(width, table[&word])?;

            if table.len() == max_entries && max_entries > 256 {
                table = (0..=255).map(|i| (vec![i as u8], i)).collect();
            }

            if table.len() < max_entries {
                word.push(byte);
                table.insert(std::mem::take(&mut word), table.len() as u64);

                if table.len() >= 1 << width {
                    width += 1;
                }
            }

            word = vec![byte];
        }
    }

    if !word.is_empty() {
        writer.write(width, table[&word])?;
    }

    writer.byte_align()?;

    Ok(())
}

pub fn compress_file<P: AsRef<Path>>(
    input_path: P,
    output_path: P,
    table_size: u8,
    fixed: bool,
) -> Result<(), std::io::Error> {
    let fin = File::open(&input_path)?;
    let mut reader = BufReader::with_capacity(32 * 1024, fin);

    let fout: File = File::create(&output_path)?;
    let writer = BufWriter::with_capacity(32 * 1024, fout);
    let mut writer = BitWriter::endian(writer, BigEndian);

    if table_size == 0 {
        compress_with_infinite_table(&mut reader, &mut writer)
    } else {
        if fixed {
            compress_with_fixed_table(&mut reader, &mut writer, table_size)
        } else {
            compress_with_resizable_table(&mut reader, &mut writer, table_size)
        }
    }
}
