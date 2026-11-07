use std::{
  fmt::{Debug, Display},
  path::{Path, PathBuf},
};

use anyhow::Result;

use serde::{Deserialize, Serialize};

use crate::config::{PracticeSection, TukaiLayoutName, TypingDuration};
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

/// Legacy storage data type
///
/// Bincode tuple format used before the code practice section
/// was introduced.
type StorageDataV0 = (Vec<Stat>, TypingDuration, TukaiLayoutName, bool, usize);

/// Storage data
///
/// Represents data saved on a device's secondary memory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageData {
  // Stored stats history
  pub stats: Vec<Stat>,

  // Selected typing duration
  pub typing_duration: TypingDuration,

  // Selected layout name
  pub layout_name: TukaiLayoutName,

  // Has transparent background
  pub has_transparent_bg: bool,

  // Selected human language index
  pub language_index: usize,

  // Selected programming language index
  pub code_index: usize,

  // Active practice section
  pub practice_section: PracticeSection,
}

/// Default data for storage
///
/// Represents the initial or fallback data used in storage.
static DEFAULT_STORAGE_DATA: StorageData = StorageData {
  stats: Vec::<Stat>::new(),
  typing_duration: TypingDuration::Minute,
  layout_name: TukaiLayoutName::Iced,
  has_transparent_bg: false,
  language_index: 0,
  code_index: 0,
  practice_section: PracticeSection::Languages,
};

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
  ///
  /// Legacy tuple files are migrated to the current format
  /// and rewritten in place.
  pub fn init(mut self) -> Result<Self> {
    if !self.file_path.exists() {
      self.init_empty_data()?;
      return Ok(self);
    }

    let data_bytes = FileHandler::read_bytes_from_file(&self.file_path)?;

    match Self::deserialize_storage_data(&data_bytes) {
      Some((data, migrated)) => {
        self.data = Some(data);

        if migrated {
          self.flush()?;
        }
      }
      None => self.init_empty_data()?,
    };

    Ok(self)
  }

  /// Deserializes storage data bytes
  ///
  /// Returns the data and whether it was migrated from the
  /// legacy tuple format.
  fn deserialize_storage_data(data_bytes: &[u8]) -> Option<(StorageData, bool)> {
    if let Ok(data) = bincode::deserialize::<StorageData>(data_bytes) {
      return Some((data, false));
    }

    // Legacy tuple format, optionally wrapped in an Option
    // (older `flush` versions serialized the Option wrapper)
    if let Ok(Some(legacy_data)) = bincode::deserialize::<Option<StorageDataV0>>(data_bytes) {
      return Some((Self::migrate_legacy_data(legacy_data), true));
    }

    if let Ok(legacy_data) = bincode::deserialize::<StorageDataV0>(data_bytes) {
      return Some((Self::migrate_legacy_data(legacy_data), true));
    }

    None
  }

  /// Migrates the legacy tuple format into the current one
  ///
  /// Keeps the stats history. Out-of-range language indexes
  /// are clamped by `Language::current_index`.
  fn migrate_legacy_data(legacy_data: StorageDataV0) -> StorageData {
    StorageData {
      stats: legacy_data.0,
      typing_duration: legacy_data.1,
      layout_name: legacy_data.2,
      has_transparent_bg: legacy_data.3,
      language_index: legacy_data.4,
      code_index: 0,
      practice_section: PracticeSection::default(),
    }
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
    let stats = &self.get_data().stats;

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
    let stats = &self.get_data().stats;

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
    let stats = &self.get_data().stats;
    stats.iter().rev().cloned().collect::<Vec<Stat>>()
  }

  /// Returns stats sorted by average WPM
  ///
  /// Used to determine the `best score`.
  pub fn get_data_stats_best(&self) -> Vec<Stat> {
    let mut data = self.get_data().stats.clone();
    data.sort_by_key(|b| std::cmp::Reverse(b.get_average_wpm()));
    data
  }

  /// Returns a TypingDuration
  pub fn get_typing_duration(&self) -> TypingDuration {
    self.get_data().typing_duration.clone()
  }

  /// Returns an active layout name
  pub fn get_layout_name(&self) -> TukaiLayoutName {
    self.get_data().layout_name.clone()
  }

  /// Returns a current human language index
  pub fn get_language_index(&self) -> usize {
    self.get_data().language_index
  }

  /// Returns a current programming language index
  pub fn get_code_index(&self) -> usize {
    self.get_data().code_index
  }

  /// Returns the active practice section
  pub fn get_practice_section(&self) -> PracticeSection {
    self.get_data().practice_section
  }

  /// Returns if has a transparend background
  pub fn get_has_transparent_bg(&self) -> bool {
    self.get_data().has_transparent_bg
  }

  /// Serialize `StorageData` into a bytes.
  ///
  /// Flushes all serialized data to the storage file.
  ///
  /// The inner data is serialized directly, without the
  /// `Option` wrapper.
  pub fn flush(&self) -> Result<()> {
    let data_bytes = bincode::serialize(self.get_data())?;
    FileHandler::write_bytes_into_file(&self.file_path, &data_bytes)
  }

  /// Insert a new cloned Stat record to the storage file.
  ///
  /// Then try to flush this record
  pub fn insert_into_stats(&mut self, stat: &Stat) -> bool {
    if let Some(storage_data) = self.get_data_mut() {
      storage_data.stats.push(stat.clone());
    }

    self.flush().is_ok()
  }

  /// Sets a new typing duration
  pub fn set_typing_duration(&mut self, typin_duration: TypingDuration) {
    if let Some(storage_data) = self.get_data_mut() {
      storage_data.typing_duration = typin_duration;
    }
  }

  /// Sets a new active layout name
  pub fn set_layout(&mut self, layout_name_changed: TukaiLayoutName) {
    if let Some(storage_data) = self.get_data_mut() {
      storage_data.layout_name = layout_name_changed;
    }
  }

  /// Sets a new human language_index
  pub fn set_language_index(&mut self, language_index: usize) {
    if let Some(storage_data) = self.get_data_mut() {
      storage_data.language_index = language_index;
    }
  }

  /// Sets a new programming language index
  pub fn set_code_index(&mut self, code_index: usize) {
    if let Some(storage_data) = self.get_data_mut() {
      storage_data.code_index = code_index;
    }
  }

  /// Sets a new active practice section
  pub fn set_practice_section(&mut self, practice_section: PracticeSection) {
    if let Some(storage_data) = self.get_data_mut() {
      storage_data.practice_section = practice_section;
    }
  }

  /// Toggles a background transparency
  pub fn set_transparent_bg(&mut self, state: bool) {
    if let Some(storage_data) = self.get_data_mut() {
      storage_data.has_transparent_bg = state;
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
    Stat::new(TypingDuration::Minute, 80, 5, 6, PracticeSection::Languages)
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
      .expect("Error occurred while deleting file");
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
      "Insert into the storage error occurred"
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
    //   .expect("Error occurred while deleting file");
  }

  #[test]
  fn flush_data() {
    let mut storage_handler = get_storage_handler();
    storage_handler.insert_into_stats(&get_test_stat());

    assert!(storage_handler.flush().is_ok());

    storage_handler
      .delete_file()
      .expect("Error occurred while deleting file");
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
      .expect("Error occurred while deleting file");
  }

  #[test]
  // Persist the code section selection into the storage file
  //
  // A second StorageHandler instance must read
  // the same values back from the file
  fn storage_persist_code_section() {
    let file_path = format!("tests/{}.tukai", Uuid::new_v4());

    let stat = Stat::new(TypingDuration::Minute, 80, 5, 6, PracticeSection::Code);
    let timestamp_secs = stat.get_timestamp_secs();

    assert!(timestamp_secs > 0);

    {
      let mut storage_handler = StorageHandler::new(&file_path)
        .init()
        .expect("Failed to initialize storage file");

      storage_handler.set_code_index(1);
      storage_handler.set_practice_section(PracticeSection::Code);
      storage_handler.insert_into_stats(&stat);

      assert!(storage_handler.flush().is_ok());
    }

    let storage_handler = StorageHandler::new(&file_path)
      .init()
      .expect("Failed to initialize storage file");

    assert_eq!(storage_handler.get_code_index(), 1);
    assert_eq!(storage_handler.get_practice_section(), PracticeSection::Code);

    let stats = storage_handler.get_data_stats_reversed();

    assert_eq!(stats.len(), 1);
    assert_eq!(stats[0].get_practice_section(), PracticeSection::Code);
    assert_eq!(stats[0].get_timestamp_secs(), timestamp_secs);

    storage_handler
      .delete_file()
      .expect("Error occurred while deleting file");
  }

  #[test]
  // Write a legacy tuple file directly
  //
  // Init must migrate it, preserving the stats history,
  // and rewrite the file in the current format
  fn storage_migrate_legacy_format() {
    let file_path = format!("tests/{}.tukai", Uuid::new_v4());

    let full_path = dirs::data_local_dir()
      .unwrap_or(PathBuf::from("/tmp"))
      .join("tukai")
      .join(&file_path);

    let stat = get_test_stat();
    let legacy_data: StorageDataV0 = (
      vec![stat],
      TypingDuration::ThirtySec,
      TukaiLayoutName::Iced,
      true,
      3,
    );

    let data_bytes = bincode::serialize(&legacy_data).unwrap();
    FileHandler::write_bytes_into_file(&full_path, &data_bytes)
      .expect("Error occurred while writing legacy storage file");

    let storage_handler = StorageHandler::new(&file_path)
      .init()
      .expect("Failed to initialize storage file");

    assert_eq!(storage_handler.get_data().stats.len(), 1);
    assert_eq!(storage_handler.get_language_index(), 3);
    assert_eq!(storage_handler.get_code_index(), 0);
    assert_eq!(
      storage_handler.get_practice_section(),
      PracticeSection::Languages
    );
    assert_eq!(
      storage_handler.get_data().stats[0].get_practice_section(),
      PracticeSection::Languages
    );
    assert_eq!(storage_handler.get_typing_duration().as_seconds(), 30);
    assert!(storage_handler.get_has_transparent_bg());

    // The file was rewritten in the current format:
    // a new instance reads the migrated data back
    let storage_handler = StorageHandler::new(&file_path)
      .init()
      .expect("Failed to initialize storage file");

    assert_eq!(storage_handler.get_data().stats.len(), 1);
    assert_eq!(storage_handler.get_language_index(), 3);

    storage_handler
      .delete_file()
      .expect("Error occurred while deleting file");
  }
}
