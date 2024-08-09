use anyhow::Result;
use clap::Parser;
use std::fs;

mod count_bytes;
mod count_chars;
mod count_lines;
mod count_words;

#[derive(Debug, PartialEq)]
pub struct FileStats {
    pub lines: usize,
    pub words: usize,
    pub bytes: usize,
    pub chars: usize,
    pub content: String,
}

pub trait Count {
    fn count(&self) -> Vec<(String, usize)>;
}

#[derive(Debug, Parser)]
#[command(name="rswc", version="0.0.1", author, about, long_about = None)]
pub struct Options {
    #[arg(short = 'L', long)]
    /// Write the length of the line containing the most bytes (default) or characters (when -m is provided to standard output.
    pub longest: bool,

    /// The number of lines in each input file is written to the standard output.
    #[arg(short, long)]
    pub lines: bool,

    /// The number of bytes in each input file is written to the standard output.
    #[arg(short, long)]
    pub count: bool,

    /// The number of characters in each input file is written to the standard output.
    #[arg(short = 'm', long)]
    pub chars: bool,

    /// The number of words in each input file is written to the standard output.
    #[arg(short = 'w', long)]
    pub words: bool,
}

// read_files read file content by file name, return file_name -> contnent vector
pub fn read_files(fiels: Vec<String>) -> Result<Vec<(String, String)>> {
    let mut files: Vec<(String, String)> = Vec::new();
    for file in fiels {
        let content = fs::read_to_string(file.clone())?;
        files.push((file, content));
    }

    Ok(files)
}
