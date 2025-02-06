use std::{fs::{File, OpenOptions}, io::{Read, Write}, path::PathBuf};

fn main() {
    println!("Hello, world!");
}

impl Drop for TempFile {
    fn drop(&mut self) {
        if let Err(e) = std::fs::remove_file(&self.file_path) {
            eprintln!("Error removing temporary file: {}", e);
        }
    }
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
        self.file.write_all(data)
    }

    pub fn read_to_string(&mut self) -> Result<String, std::io::Error> {
        // Your code here...
        let mut file = OpenOptions::new().read(true).open(&self.file_path)?;
        let mut file_content = String::new();
        let _ = file.read_to_string(&mut file_content)?;
        Ok(file_content)

    }

    pub fn path(&self) -> &PathBuf {
        &self.file_path
    }

    pub fn file(&self) -> &File {
        &self.file
    }
}
