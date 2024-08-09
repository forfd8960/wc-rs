use crate::{Count, CountType};

pub struct CountChars {
    pub files: Vec<(String, String)>,
}

impl CountChars {
    pub fn new(files: Vec<(String, String)>) -> Self {
        Self { files }
    }
}

impl Count for CountChars {
    fn count_type(&self) -> CountType {
        CountType::Chars
    }

    fn count(&self) -> Vec<(String, usize)> {
        let mut stats: Vec<(String, usize)> = Vec::new();
        for (file, content) in self.files.iter() {
            let ch_count = content.chars().count();
            stats.push((file.to_string(), ch_count));
        }

        stats
    }
}
