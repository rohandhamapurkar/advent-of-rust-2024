use std::{fs, io::Write};

pub struct LogQuery<'a> {
    logs: &'a Vec<String>,
}

impl<'a> LogQuery<'a> {
    pub fn new(logs: &'a Vec<String>) -> Self {
        LogQuery { logs }
    }

    pub fn search(&self, keyword: &str) -> Vec<&'a String> {
        self.logs
            .iter()
            .filter(|log| log.contains(keyword))
            .collect()
    }

    pub fn export_to_file(&self, keyword: &str, path: &str) -> std::io::Result<()> {
        let mut file = fs::File::create(path)?;
        for log in self.search(keyword) {
            file.write_all(format!("{}\n", log).as_bytes())?;
        }
        Ok(())
    }
}

fn main() {}