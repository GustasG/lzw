use std::path::PathBuf;

use clap::{Parser, ValueEnum, ValueHint};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub enum Mode {
    Compress,
    Decompress,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    #[arg(long, value_hint = ValueHint::FilePath)]
    pub input_file: PathBuf,

    #[arg(long, value_hint = ValueHint::FilePath)]
    pub output_file: PathBuf,

    #[arg(long, value_enum)]
    pub mode: Mode,

    #[arg(long, default_value = "12")]
    pub length: u8,
}
