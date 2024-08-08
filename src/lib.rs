use clap::Parser;

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
