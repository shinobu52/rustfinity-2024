use std::fs::{remove_file, File, OpenOptions};
use std::path::PathBuf;
use std::str;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct TempFile {
    file_path: PathBuf,
    file: File,
    // 1. Add a new field named `content` with a valid type
    content: Vec<u8>,
}

impl TempFile {
    pub fn new() -> Result<Self, std::io::Error> {
        let temp_dir = std::env::temp_dir();

        let random_number: u64 = {
            let start = SystemTime::now();
            let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
            since_epoch.as_nanos() as u64
        };

        let file_name = format!("tempfile-{}.tmp", random_number);
        let file_path = temp_dir.join(file_name);
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&file_path)?;

        Ok(Self {
            file_path,
            file,
            content: "".into(),
        })
    }

    // 2. Change this method to update the `content` field on every write
    pub fn write(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        self.content = data.to_vec();
        Ok(())
    }

    pub fn read_from_cache(&self) -> &str {
        // 3. Update this method to return the content as a string slice
        str::from_utf8(&self.content).unwrap_or("")
    }

    pub fn read_to_string(&mut self) -> Result<String, std::io::Error> {
        let read_str = self.read_from_cache();
        Ok(read_str.to_string())
    }

    pub fn path(&self) -> &PathBuf {
        &self.file_path
    }

    pub fn file(&self) -> &File {
        &self.file
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = remove_file(&self.file_path);
    }
}
