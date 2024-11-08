use std::{fs::{File, OpenOptions}, io::{Read, Write}, path::{Path, PathBuf}};

pub struct FileHandler {

}

impl FileHandler {
  fn open_file<P: AsRef<Path>>(path: P) -> Result<File, std::io::Error> {
    OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(path)
  }

  pub fn write_bytes_into_file<P: AsRef<Path>>(path: P, bytes: &[u8]) -> Result<(), std::io::Error> {
    let mut file = FileHandler::open_file(path)?;
    file.write_all(bytes)?;

    Ok(())
  }

  pub fn read_bytes_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, std::io::Error> {
    let mut bytes_buf = Vec::new();

    let mut file = FileHandler::open_file(path)?;

    file.read_to_end(&mut bytes_buf)?;

    Ok(bytes_buf)
  }
}
