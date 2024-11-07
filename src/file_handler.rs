use std::{fs::{File, OpenOptions}, io::{Read, Write}, path::PathBuf};

pub struct FileHandler {

}

impl FileHandler {
  //io::Result<File>
  fn open_file(path: PathBuf) -> Result<File, std::io::Error> {
    OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(path)
  }

  pub fn write_bytes_into_file(path: PathBuf, bytes: &[u8]) -> Result<(), std::io::Error> {
    let mut file = FileHandler::open_file(path)?;
    file.write_all(bytes)?;

    Ok(())
  }

  pub fn read_bytes_from_file(path: PathBuf) -> Result<Vec<u8>, std::io::Error> {
    let mut bytes_buf = Vec::new();

    let mut file = FileHandler::open_file(path)?;

    file.read(&mut bytes_buf)?;

    Ok(bytes_buf)
  }
}
