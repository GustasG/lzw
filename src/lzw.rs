use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, ErrorKind, Read, Write};
use std::path::Path;

use bitstream_io::{BigEndian, BitRead, BitReader, BitWrite, BitWriter};

fn compress<R: Read, W: BitWrite>(
    reader: &mut R,
    writer: &mut W,
    length: u8,
) -> Result<(), std::io::Error> {
    if !(8..=16).contains(&length) {
        return Err(std::io::Error::new(
            ErrorKind::InvalidInput,
            format!("length must be between 8 and 16, got {}", length),
        ));
    }

    let max_entries = 2_usize.pow(length as u32);
    let mut table = (0..=255)
        .map(|i| (vec![i as u8], i as u16))
        .collect::<HashMap<Vec<u8>, u16>>();

    let mut word = Vec::new();

    writer.write(8, length)?;

    for byte in reader.bytes() {
        let byte = byte?;
        word.push(byte);

        if !table.contains_key(&word) {
            word.pop();
            writer.write(length as u32, table[&word])?;

            if table.len() == max_entries && max_entries > 256 {
                table = (0..=255).map(|i| (vec![i as u8], i as u16)).collect();
            }

            if table.len() < max_entries {
                word.push(byte);
                table.insert(std::mem::take(&mut word), table.len() as u16);
            }

            word = vec![byte];
        }
    }

    if !word.is_empty() {
        writer.write(length as u32, table[&word])?;
    }

    writer.byte_align()?;

    Ok(())
}

fn decompress<R: BitRead, W: Write>(reader: &mut R, writer: &mut W) -> Result<(), std::io::Error> {
    let length = reader.read::<u8>(8)?;

    if !(8..=16).contains(&length) {
        return Err(std::io::Error::new(
            ErrorKind::InvalidData,
            "invalid length",
        ));
    }

    let max_entries = 2_usize.pow(length as u32);
    let mut table = (0..=255)
        .map(|i| (i as u16, vec![i as u8]))
        .collect::<HashMap<u16, Vec<u8>>>();

    let mut prev_code = match reader.read::<u16>(length as u32) {
        Ok(code) => code,
        Err(e) => match e.kind() {
            ErrorKind::UnexpectedEof => return Ok(()),
            _ => return Err(e),
        },
    };

    writer.write_all(&table[&prev_code])?;

    loop {
        match reader.read::<u16>(length as u32) {
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
                    table = (0..=255).map(|i| (i as u16, vec![i as u8])).collect();
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

pub fn compress_file<P: AsRef<Path>>(
    input_path: P,
    output_path: P,
    max_code_size: u8,
) -> Result<(), std::io::Error> {
    let fin = File::open(&input_path)?;
    let mut reader = BufReader::with_capacity(32 * 1024, fin);

    let fout: File = File::create(&output_path)?;
    let writer = BufWriter::with_capacity(32 * 1024, fout);
    let mut writer = BitWriter::endian(writer, BigEndian);

    match compress(&mut reader, &mut writer, max_code_size) {
        Ok(()) => Ok(()),
        Err(e) => {
            std::fs::remove_file(output_path).ok();
            Err(e)
        }
    }
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

    match decompress(&mut reader, &mut writer) {
        Ok(()) => Ok(()),
        Err(e) => {
            std::fs::remove_file(output_path).ok();
            Err(e)
        }
    }
}
