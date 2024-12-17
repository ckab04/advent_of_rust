use std::{fs::File, io::Write};

fn main() {
    println!("Hello, world!");
}

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
        // 🎁 Your code here! 🎁
        //

        let mut file = File::create(path)?;
        let all_logs = self.search(keyword);

        // all_logs.into_iter()
        //     .for_each(|f| file.write_all(f.as_bytes()).unwrap());
        //

        for log in all_logs{
            //file.write_all(log.as_bytes())?;
             writeln!(file, "{}", log)?;
        }

        Ok(())
    }
}
