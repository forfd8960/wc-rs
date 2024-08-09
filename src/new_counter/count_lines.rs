use std::collections::HashMap;

use crate::{CountType, CountV1};

pub struct CountLines<'a> {
    pub files: Vec<(String, &'a str)>,
}

impl<'a> CountLines<'a> {
    pub fn new(files: Vec<(String, &'a str)>) -> Self {
        Self { files }
    }
}

impl<'a> CountV1 for CountLines<'a> {
    fn count_type(&self) -> CountType {
        CountType::Lines
    }

    fn count(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        for (file, content) in self.files.iter() {
            let lines_count = content.lines().count();
            stats.insert(file.to_string(), lines_count);
        }

        stats
    }
}
