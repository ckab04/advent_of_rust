use std::{fs::File, path::PathBuf};

fn main() {
    println!("Hello, world!");
}

pub struct TempFile {

    file_path: PathBuf,
    file: File
}

impl TempFile {
    pub fn new() -> Result<Self, std::io::Error> {
        // Your code here...

        let mut path = std::env::temp_dir();
        path.push("my_file");
        let file = File::create(&path)?;
        Ok(Self{
            file_path: path,
            file
        })

    }

    pub fn write(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        // Your code here...
    }

    pub fn read_to_string(&mut self) -> Result<String, std::io::Error> {
        // Your code here...
    }

    pub fn path(&self) -> &PathBuf {
        &self.file_path
    }

    pub fn file(&self) -> &File {
        &self.file
    }
}
