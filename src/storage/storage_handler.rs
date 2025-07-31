use std::{
  fmt::{Debug, Display},
  path::{Path, PathBuf},
};

use anyhow::Result;

use crate::config::{TukaiLayoutName, TypingDuration};
use crate::file_handler::FileHandler;

use super::stats::Stat;

#[derive(Debug)]
pub struct StorageHandlerError {
  message: String,
}

impl Display for StorageHandlerError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "StorageHandlerError: {}", self.message)
  }
}

impl std::error::Error for StorageHandlerError {}

#[allow(unused)]
impl StorageHandlerError {
  fn new(message: String) -> Self {
    Self { message }
  }
}

/// Storage data type
///
/// Represents types saved on a device's secondary memory.
pub type StorageData = (Vec<Stat>, TypingDuration, TukaiLayoutName, bool, usize);

/// Default data for storage
///
/// Represents the initial or fallback data used in storage.
static DEFAULT_STORAGE_DATA: StorageData = (
  Vec::<Stat>::new(),
  TypingDuration::Minute,
  TukaiLayoutName::Iced,
  false,
  0,
);

/// Represents a storage file with a specified file path
///
/// Handles both read and write operations.
pub struct StorageHandler {
  // Path to the `storage` file
  // Defaults to the OS's local directory (refer to the `dirs` library)
  file_path: PathBuf,

  // Data stored in the `storage` binary file
  data: Option<StorageData>,
}

/// Total statistics overview
///
/// Includes the average WPM (words per minute) and average accuracy.
pub struct StatOverview {
  pub total_stats_count: usize,
  pub total_average_wpm: usize,
  pub total_average_accuracy: f64,
}

impl StorageHandler {
  /// Creates a new `storage` file
  ///
  /// Uses a local directory path or `/tmp` as the default location.
  pub fn new<P: AsRef<Path>>(file_path: P) -> Self {
    let local_dir_path = dirs::data_local_dir().unwrap_or(PathBuf::from("/tmp"));

    let full_path = local_dir_path.join("tukai").join(file_path);

    Self {
      file_path: full_path,
      data: None,
    }
  }

  #[cfg(test)]
  pub fn delete_file(&self) -> Result<()> {
    std::fs::remove_file(&self.file_path)?;
    Ok(())
  }

  /// Inits empty data and write into the `storage file`
  fn init_empty_data(&mut self) -> Result<()> {
    let data = self
      .data
      .get_or_insert_with(|| DEFAULT_STORAGE_DATA.clone());

    let data_bytes = bincode::serialize(&data).unwrap();
    FileHandler::write_bytes_into_file(&self.file_path, &data_bytes)?;

    Ok(())
  }

  /// Inits the storage
  ///
  /// Try to read all bytes from the storage file
  /// Then set into the data
  pub fn init(mut self) -> Result<Self> {
    if !self.file_path.exists() {
      self.init_empty_data()?;
      return Ok(self);
    }

    let data_bytes = FileHandler::read_bytes_from_file(&self.file_path)?;

    match bincode::deserialize(&data_bytes) {
      Ok(data) => self.data = data,
      Err(_) => self.init_empty_data()?,
    };

    Ok(self)
  }

  /// Returns data from the storage
  ///
  /// If data is None, returns the storage's default values.
  pub fn get_data(&self) -> &StorageData {
    if let Some(storage_data) = &self.data {
      storage_data
    } else {
      &DEFAULT_STORAGE_DATA
    }
  }

  /// Returns data from the storage as mutable
  pub fn get_data_mut(&mut self) -> Option<&mut StorageData> {
    self.data.as_mut()
  }

  /// Returns the complete statistics overview
  ///
  /// (average WPM, average accuracy)
  pub fn get_data_for_overview(&self) -> StatOverview {
    let stats = &self.get_data().0;

    let (sum_wpm, sum_accuracy) = stats.iter().fold((0, 0.0), |(wpm, acc), stat| {
      (wpm + stat.get_average_wpm(), acc + stat.get_accuracy())
    });

    let accuracy = (sum_accuracy / stats.len() as f64).round();

    StatOverview {
      total_stats_count: stats.len(),
      total_average_wpm: sum_wpm.checked_div(stats.len()).unwrap_or(0),
      total_average_accuracy: if accuracy.is_nan() { 0.0 } else { accuracy },
    }
  }

  /// Returns data for the chart widget
  ///
  /// Creates a dataset for the chart and calculates the best WPM.
  pub fn get_data_for_chart(&self) -> (usize, Vec<(f64, f64)>) {
    let stats = &self.get_data().0;

    let mut best_wpm = 0_usize;
    let dataset = stats
      .iter()
      .enumerate()
      .rev()
      .map(|(index, stat)| {
        let stat_wpm = stat.get_average_wpm();

        best_wpm = best_wpm.max(stat_wpm);

        (index as f64, stat_wpm as f64)
      })
      .collect::<Vec<(f64, f64)>>();

    (best_wpm.max(100), dataset)
  }

  /// Returns stats in reversed order
  ///
  /// Newest first
  pub fn get_data_stats_reversed(&self) -> Vec<Stat> {
    let stats = &self.get_data().0;
    stats.iter().rev().cloned().collect::<Vec<Stat>>()
  }

  /// Returns stats sorted by average WPM
  ///
  /// Used to determine the `best score`.
  pub fn get_data_stats_best(&self) -> Vec<Stat> {
    let mut data = self.get_data().0.clone();
    data.sort_by_key(|b| std::cmp::Reverse(b.get_average_wpm()));
    data
  }

  /// Returns a TypingDuration
  pub fn get_typing_duration(&self) -> TypingDuration {
    self.get_data().1.clone()
  }

  /// Returns an active layout name
  pub fn get_layout_name(&self) -> TukaiLayoutName {
    self.get_data().2.clone()
  }

  /// Returns a current language index
  pub fn get_language_index(&self) -> usize {
    self.get_data().4
  }

  /// Returns if has a transparend background
  pub fn get_has_transparent_bg(&self) -> bool {
    self.get_data().3
  }

  /// Serialize `StorageData` into a bytes.
  ///
  /// Flushes all serialized data to the storage file.
  pub fn flush(&self) -> Result<()> {
    let data_bytes = bincode::serialize(&self.data)?;
    FileHandler::write_bytes_into_file(&self.file_path, &data_bytes)
  }

  /// Insert a new cloned Stat record to the storage file.
  ///
  /// Then try to flush this record
  pub fn insert_into_stats(&mut self, stat: &Stat) -> bool {
    if let Some(storage_data) = self.get_data_mut() {
      storage_data.0.push(stat.clone());
    }

    self.flush().is_ok()
  }

  /// Sets a new typing duration
  pub fn set_typing_duration(&mut self, typin_duration: TypingDuration) {
    if let Some(storage_data) = self.get_data_mut() {
      storage_data.1 = typin_duration;
    }
  }

  /// Sets a new active layout name
  pub fn set_layout(&mut self, layout_name_changed: TukaiLayoutName) {
    if let Some(storage_data) = self.get_data_mut() {
      storage_data.2 = layout_name_changed;
    }
  }

  /// Sets a new language_index
  pub fn set_language_index(&mut self, language_index: usize) {
    if let Some(storage_data) = self.get_data_mut() {
      storage_data.4 = language_index;
    }
  }

  /// Toggles a background transparency
  pub fn set_transparent_bg(&mut self, state: bool) {
    if let Some(storage_data) = self.get_data_mut() {
      storage_data.3 = state;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::config::TypingDuration;
  use uuid::Uuid;

  fn get_storage_handler() -> StorageHandler {
    let storage_helper = StorageHandler::new(format!("tests/{}.tukai", Uuid::new_v4()))
      .init()
      .expect("Failed to initialize storage file");

    storage_helper
  }

  fn get_test_stat() -> Stat {
    Stat::new(TypingDuration::Minute, 80, 5)
  }

  //#[test]
  // fn storage_local_dir_initialization() {
  //   let storage_handler = get_storage_handler();
  // }

  #[test]
  // Just validate if binary file was created right
  fn storage_load() {
    let storage_handler = get_storage_handler();
    let _storage_data = storage_handler.get_data();

    storage_handler
      .delete_file()
      .expect("Error occured while deleting file");
  }

  #[test]
  // Init an empty storage data
  //
  // Insert test Stat into the file
  //
  // Try to reverse read from the memory
  fn storage_insert_into_data_stats() {
    let mut storage_handler = get_storage_handler();

    let stat = get_test_stat();

    assert!(
      storage_handler.insert_into_stats(&stat),
      "Insert into the storage error occured"
    );

    let _stats = storage_handler.get_data_stats_reversed();

    // assert!(
    //   stats.is_some(),
    //   "Failed to read from the storage stats (stats is None)"
    // );
    //
    // let stats_unwraped = stats.unwrap();
    //
    // let stat_from_memory = &stats_unwraped[0];
    //
    // assert_eq!(stat_from_memory.get_average_wpm(), stat.get_average_wpm());
    //
    // storage_handler
    //   .delete_file()
    //   .expect("Error occured while deleting file");
  }

  #[test]
  fn flush_data() {
    let mut storage_handler = get_storage_handler();
    storage_handler.insert_into_stats(&get_test_stat());

    assert!(storage_handler.flush().is_ok());

    storage_handler
      .delete_file()
      .expect("Error occured while deleting file");
  }

  #[test]
  fn load_flushed_data() {
    let mut storage_handler = get_storage_handler();
    storage_handler.insert_into_stats(&get_test_stat());

    println!("{:?}", storage_handler.get_data());

    let data = storage_handler.get_data();
    println!("{:?}", data);

    storage_handler
      .delete_file()
      .expect("Error occured while deleting file");
  }
}
