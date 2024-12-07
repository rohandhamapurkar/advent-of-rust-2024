pub struct LogQuery<'a> {
    logs: &'a Vec<String>,
}

// 1. Finish the implementation of LogQuery
impl<'a> LogQuery<'a> {
    pub fn new(l: &'a Vec<String>) -> Self {
        LogQuery { logs: l }
    }

    pub fn search(&self, keyword: &str) -> Vec<&String> {
        let mut v = vec![];
        for s in self.logs {
            if s.contains(keyword) {
                v.push(s);
            }
        }
        return v
    }
}

fn main() {}
