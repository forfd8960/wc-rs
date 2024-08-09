use anyhow::Result;
use clap::Parser;
use std::{collections::HashMap, fs};

use count_bytes::CountBytes;
use count_chars::CountChars;
use count_lines::CountLines;
use count_words::CountWords;

mod count_bytes;
mod count_chars;
mod count_lines;
mod count_words;

mod new_counter;

#[derive(Debug, PartialEq)]
pub struct FileStats {
    pub lines: usize,
    pub words: usize,
    pub bytes: usize,
    pub chars: usize,
    pub file: String,
}

#[derive(Debug, PartialEq)]
pub enum CountType {
    Lines,
    Words,
    Bytes,
    Chars,
}

pub trait Count {
    fn count(&self) -> Vec<(String, usize)>;
    fn count_type(&self) -> CountType;
}

pub trait CountV1 {
    fn count(&self) -> HashMap<String, usize>;
    fn count_type(&self) -> CountType;
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

    #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    pub files: Vec<String>,
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

impl FileStats {
    pub fn new(file: String) -> Self {
        Self {
            file,
            lines: 0,
            words: 0,
            bytes: 0,
            chars: 0,
        }
    }
}

pub struct OptionsHandler {
    options: Options,
    counters: Vec<Box<dyn Count>>,
}

impl OptionsHandler {
    pub fn new(options: Options) -> Self {
        Self {
            options,
            counters: vec![],
        }
    }

    pub fn handle_options(&mut self) -> anyhow::Result<()> {
        if self.options.files.len() == 0 {
            return Ok(());
        }

        let res = self.count_files()?;
        for (file, stats) in res.iter() {
            println!(
                "{}: {} {} {} {}",
                file, stats.lines, stats.words, stats.bytes, stats.chars
            );
        }
        Ok(())
    }

    fn count_stdin(&self) -> Result<Vec<FileStats>> {
        Ok(vec![])
    }

    fn count_files(&mut self) -> Result<HashMap<String, FileStats>> {
        self.build_counters()?;

        let mut file_stats: HashMap<String, FileStats> = HashMap::new();

        for counter in self.counters.iter() {
            let stats = counter.count();
            match counter.count_type() {
                CountType::Lines => {
                    for (file, count) in stats.iter() {
                        let f = file.clone();
                        file_stats
                            .entry(f)
                            .or_insert(FileStats::new(file.clone()))
                            .lines += count;

                        println!("{}: {}", file, count);
                    }
                }
                CountType::Words => {
                    for (file, count) in stats.iter() {
                        let f = file.clone();
                        file_stats
                            .entry(f)
                            .or_insert(FileStats::new(file.clone()))
                            .words += count;

                        println!("{}: {}", file, count);
                    }
                }
                CountType::Chars => {
                    for (file, count) in stats.iter() {
                        let f = file.clone();
                        file_stats
                            .entry(f)
                            .or_insert(FileStats::new(file.clone()))
                            .chars += count;

                        println!("{}: {}", file, count);
                    }
                }
                CountType::Bytes => {
                    for (file, count) in stats.iter() {
                        let f = file.clone();
                        file_stats
                            .entry(f)
                            .or_insert(FileStats::new(file.clone()))
                            .bytes += count;

                        println!("{}: {}", file, count);
                    }
                }
            }
        }

        Ok(file_stats)
    }

    fn build_counters(&mut self) -> Result<()> {
        let files = self.options.files.clone();
        let files_content = read_files(files)?;

        let lines_counter = CountLines::new(files_content.clone());
        if self.options.lines {
            self.counters.push(Box::new(lines_counter));
        }

        let words_counter = CountWords::new(files_content.clone());
        if self.options.words {
            self.counters.push(Box::new(words_counter));
        }

        let chars_counter = CountChars::new(files_content.clone());
        if self.options.chars {
            self.counters.push(Box::new(chars_counter));
        }

        let bytes_counter = CountBytes::new(files_content.clone());
        if self.options.count {
            self.counters.push(Box::new(bytes_counter));
        }

        Ok(())
    }
}
