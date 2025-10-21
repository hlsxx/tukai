use std::{
  fs::{File, OpenOptions, create_dir_all},
  io::{Read, Write},
  path::Path,
};

use anyhow::{Error, Result};

pub struct FileHandler;

impl FileHandler {
  /// Opens a file for reading and writing, creating it if it does not exist.
  ///
  /// If the file at the specified path exists, it is opened with read/write access.
  /// If it does not exist, a new file is created.
  ///
  /// # Parameters
  /// - `path`: A path to the file to open or create.
  ///
  /// # Returns
  /// A [`Result`] containing the opened [`File`] on success, or an error on failure.
  #[allow(clippy::suspicious_open_options)]
  fn open_file<P: AsRef<Path>>(path: P) -> Result<File> {
    let path_buf = path.as_ref().to_path_buf();

    if let Some(parent_dir) = path_buf.parent() {
      create_dir_all(parent_dir)?;
    }

    OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(path)
      .map_err(Error::from)
  }

  /// Writes the given bytes into a file at the specified path.
  ///
  /// This function opens the file (creating it if it doesn’t exist),
  /// writes all the provided bytes to it, and then closes the file.
  ///
  /// # Parameters
  /// - `path`: The path to the file where the bytes will be written.
  /// - `bytes`: A byte slice containing the data to write.
  ///
  /// # Returns
  /// A [`Result`] which is [`Ok`] if the operation succeeds,
  /// or an error if opening or writing to the file fails.
  pub fn write_bytes_into_file<P: AsRef<Path>>(path: P, bytes: &[u8]) -> Result<()> {
    let mut file = FileHandler::open_file(path)?;
    file.write_all(bytes)?;
    Ok(())
  }

  /// Reads all bytes from the file at the specified path into a buffer.
  ///
  /// This function opens the file for reading, reads its entire contents
  /// into a `Vec<u8>`, and returns the buffer.
  ///
  /// # Parameters
  /// - `path`: The path to the file to read from.
  ///
  /// # Returns
  /// A [`Result`] containing a vector of bytes read from the file,
  /// or an error if the file could not be opened or read.
  pub fn read_bytes_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let mut bytes_buf = Vec::new();
    let mut file = FileHandler::open_file(path)?;
    file.read_to_end(&mut bytes_buf)?;
    Ok(bytes_buf)
  }
}
