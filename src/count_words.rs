use crate::{Count, CountType};

pub struct CountWords {
    pub files: Vec<(String, String)>,
}

impl CountWords {
    pub fn new(files: Vec<(String, String)>) -> Self {
        Self { files }
    }
}

impl Count for CountWords {
    fn count_type(&self) -> CountType {
        CountType::Words
    }

    fn count(&self) -> Vec<(String, usize)> {
        let mut stats: Vec<(String, usize)> = Vec::new();
        for (file, content) in self.files.iter() {
            let words_count = content.split_ascii_whitespace().count();
            stats.push((file.to_string(), words_count));
        }

        stats
    }
}
