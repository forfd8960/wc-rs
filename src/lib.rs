use anyhow::Result;
use clap::Parser;
use new_counter::count_files;
use std::fs;

mod new_counter;

#[derive(Debug, PartialEq)]
pub struct FileStats {
    pub lines: usize,
    pub words: usize,
    pub bytes: usize,
    pub chars: usize,
    pub file: String,
}

#[derive(Debug, Parser, Clone)]
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
pub fn read_files(file_names: &Vec<String>) -> Result<Vec<(String, String)>> {
    let mut file_content: Vec<(String, String)> = Vec::new();
    for file in file_names {
        let content = fs::read_to_string(file.clone())?;
        file_content.push((file.to_owned(), content));
    }

    Ok(file_content)
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
}

impl OptionsHandler {
    pub fn new(options: Options) -> Self {
        Self { options }
    }

    pub fn handle_options(&self) -> anyhow::Result<()> {
        if self.options.files.len() == 0 {
            return Ok(());
        }

        let res = count_files(&self.options)?;
        for file_stat in res.iter() {
            println!(
                "{}: {} {} {} {}",
                file_stat.file, file_stat.lines, file_stat.words, file_stat.bytes, file_stat.chars
            );
        }
        Ok(())
    }

    fn count_stdin(&self) -> Result<Vec<FileStats>> {
        Ok(vec![])
    }
}
