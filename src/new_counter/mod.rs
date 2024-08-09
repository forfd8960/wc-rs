use anyhow::Result;

use crate::{read_files, FileStats, Options};

pub fn count_files(opts: &Options) -> Result<Vec<FileStats>> {
    let mut result = vec![];
    let file_content = read_files(&opts.files)?;
    for (file, content) in file_content {
        let mut stats = FileStats {
            file,
            bytes: 0,
            chars: 0,
            lines: 0,
            words: 0,
        };

        if opts.chars {
            stats.chars = content.chars().count();
        }

        if opts.count {
            stats.bytes = content.len();
        }

        if opts.lines {
            stats.lines = content.lines().count()
        }
        if opts.words {
            stats.words = content.split_ascii_whitespace().count();
        }
        result.push(stats);
    }

    Ok(result)
}
