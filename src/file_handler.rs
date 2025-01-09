use std::{
  fs::{create_dir_all, File, OpenOptions},
  io::{Read, Write},
  path::Path,
};

pub struct FileHandler;

impl FileHandler {
  /// Opens a file for reading, writing creating if it not exist
  fn open_file<P: AsRef<Path>>(path: P) -> Result<File, std::io::Error> {
    let path_buf = path.as_ref().to_path_buf();

    if let Some(parent_dir) = path_buf.parent() {
      create_dir_all(parent_dir)?;
    }

    OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(path)
  }

  /// Writes bytes into the file
  pub fn write_bytes_into_file<P: AsRef<Path>>(
    path: P,
    bytes: &[u8],
  ) -> Result<(), std::io::Error> {
    let mut file = FileHandler::open_file(path)?;
    file.write_all(bytes)?;

    Ok(())
  }

  /// Reads bytes into a buffer from the file
  pub fn read_bytes_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, std::io::Error> {
    let mut bytes_buf = Vec::new();

    let mut file = FileHandler::open_file(path)?;

    file.read_to_end(&mut bytes_buf)?;

    Ok(bytes_buf)
  }
}
