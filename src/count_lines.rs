use crate::{Count, CountType};

pub struct CountLines {
    pub files: Vec<(String, String)>,
}

impl CountLines {
    pub fn new(files: Vec<(String, String)>) -> Self {
        Self { files }
    }
}

impl Count for CountLines {
    fn count_type(&self) -> CountType {
        CountType::Lines
    }

    fn count(&self) -> Vec<(String, usize)> {
        let mut stats: Vec<(String, usize)> = Vec::new();
        for (file, content) in self.files.iter() {
            let lines = content.lines().count();
            stats.push((file.to_string(), lines));
        }

        stats
    }
}
