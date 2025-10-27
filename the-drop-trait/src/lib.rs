use std::{fs::File, path::PathBuf};

pub struct TempFile {
    pub path: PathBuf,
}

impl TempFile {
    pub fn new<T: AsRef<str>>(path: T) -> Result<TempFile, std::io::Error> {
        match File::create(path.as_ref()) {
            Ok(_) => Ok(TempFile {
                path: PathBuf::from(path.as_ref()),
            }),
            Err(e) => Err(e),
        }
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

// Example usage
pub fn main() {
    let file_path = PathBuf::from("example_temp_file.tmp");
    let tempfile =
        TempFile::new(file_path.to_str().unwrap()).expect("Failed to create temporary file");

    assert!(tempfile.path.exists(), "File does not exist");

    drop(tempfile);

    assert!(!file_path.exists(), "File was not deleted");

    let tempfile_2 = TempFile::new(&String::from("example_temp_file_2.tmp"))
        .expect("Failed to create temporary file");

    drop(tempfile_2);
}
