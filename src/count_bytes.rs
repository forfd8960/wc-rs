use crate::{Count, CountType};

pub struct CountBytes {
    pub files: Vec<(String, String)>,
}

impl CountBytes {
    pub fn new(files: Vec<(String, String)>) -> Self {
        Self { files }
    }
}

impl Count for CountBytes {
    fn count_type(&self) -> CountType {
        CountType::Bytes
    }

    fn count(&self) -> Vec<(String, usize)> {
        let mut stats: Vec<(String, usize)> = Vec::new();
        for (file, content) in self.files.iter() {
            let bs_count = content.bytes().count();
            stats.push((file.to_string(), bs_count));
        }

        stats
    }
}
