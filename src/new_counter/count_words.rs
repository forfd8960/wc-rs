use crate::{Count, CountType};

pub struct CountWords<'a> {
    pub files: Vec<(String, &'a str)>,
}

impl<'a> CountWords<'a> {
    pub fn new(files: Vec<(String, &'a str)>) -> Self {
        Self { files }
    }
}

impl<'a> Count for CountWords<'a> {
    fn count_type(&self) -> CountType {
        CountType::Lines
    }

    fn count(&self) -> Vec<(String, usize)> {
        let mut stats: Vec<(String, usize)> = Vec::new();
        for (file, content) in self.files.iter() {
            let lines_count = content.split_ascii_whitespace().count();
            stats.push((file.to_string(), lines_count));
        }

        stats
    }
}
