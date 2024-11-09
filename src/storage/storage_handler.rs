use std::collections::HashMap;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

use crate::file_handler::FileHandler;

use super::stats::{Stat, TypingDuration};
use super::activities::Activities;

#[derive(Deserialize, Serialize, Hash, PartialEq, Eq, Debug)]
pub enum StorageDataType {
  Stats,
  Activities
}

#[derive(Deserialize, Serialize, Debug)]
pub enum StorageDataValue {
  Stats(Vec<Stat>),
  Activites(Activities)
}

impl StorageDataValue {
  // pub fn insert_stats(stat_name: String, stat_value: i32) -> Self {
  //   StorageDataValue::Stats(Stats { stat_name, stat_value })
  // }
}

type StorageData = HashMap<StorageDataType, StorageDataValue>;

pub struct StorageHandler {
  file_path: PathBuf,
  data: StorageData
}

impl StorageHandler {

  pub fn new<P: AsRef<Path>>(file_path: P) -> Self {
    Self {
      file_path: file_path.as_ref().to_owned(),
      data: HashMap::new()
    }
  }

  /// Default data for the storage
  ///
  /// Create an empty Vec for stats
  /// Create an empty Vec for activities
  ///
  /// Store into a HashMap
  ///
  /// Writes into the binary file
  pub fn default(self) -> Result<Self, std::io::Error> {
    let mut empty_data: StorageData = HashMap::new();

    let empty_stats = StorageDataValue::Stats(Vec::new());
    let empty_activities= StorageDataValue::Activites(Vec::new());

    empty_data.insert(StorageDataType::Stats, empty_stats);
    empty_data.insert(StorageDataType::Activities, empty_activities);

    let data_bytes = bincode::serialize(&empty_data).unwrap();
    FileHandler::write_bytes_into_file(&self.file_path, &data_bytes)?;

    Ok(self)
  }

  /// Inits the storage
  ///
  /// Try to read all bytes from the storage file
  /// Then set into the data
  pub fn init(mut self) -> Self {
    let data_bytes = FileHandler::read_bytes_from_file(&self.file_path).unwrap();
    self.data = bincode::deserialize(&data_bytes).unwrap();

    self
  }

  pub fn get_data(&self) -> &StorageData {
    &self.data
  }

  /// Gets the stats from the storage
  fn get_data_stats_mut(&mut self) -> Option<&mut Vec<Stat>> {
    // println!("{:?}", self.data.);
    if let Some(StorageDataValue::Stats(stats)) = self.data.get_mut(&StorageDataType::Stats) {
      Some(stats)
    } else {
      None
    }
  }

  /// Gets the activities from the storage
  fn get_data_activities_mut(&mut self) -> Option<&Activities> {
    if let Some(StorageDataValue::Activites(activities)) = self.data.get_mut(&StorageDataType::Activities) {
      Some(activities)
    } else {
      None
    }
  }

  /// Gets the storage data from file
  ///
  /// &[0, 55, 55] -> StorageData
  fn read_data_from_file(&self) -> StorageData {
    let data_bytes = FileHandler::read_bytes_from_file(&self.file_path).unwrap();
    let data = bincode::deserialize::<StorageData>(&data_bytes).unwrap();

    data
  }

  pub fn insert_into_stats(
    &mut self,
    stat: &Stat
  ) -> bool {
    if let Some(stats) = self.get_data_stats_mut() {
      stats.push(stat.clone());
      return true;
    }

    false
  }

}

#[cfg(test)]
mod tests {
  use crate::storage::stats::TypingDuration;
  use super::*;

  fn get_storage_handler() -> StorageHandler {
    let storage_helper = StorageHandler::new("test.tukai")
      .default()
      .unwrap()
      .init();

    storage_helper
  }

  #[test]
  // Just validate if binary file was created right
  fn storage_read_from_data() {
    let storage_handler = get_storage_handler();
    let storage_data= storage_handler.read_data_from_file();

    assert!(storage_data.get(&StorageDataType::Stats).is_some(), "Stats not initialized successfully");
    assert!(storage_data.get(&StorageDataType::Activities).is_some(), "Activities not initialized successfully");
  }

  #[test]
  // Init an empty storage data
  //
  // Insert test Stat into the file
  //
  // Try to reverse read from the binary file
  fn storage_insert_into_data_stats() {
    let mut storage_handler = get_storage_handler();

    let stat = Stat::new(
      TypingDuration::Minute,
      80,
      5,
      60
    );

    assert!(storage_handler.insert_into_stats(&stat), "Insert into the storage error occured");

    let stats = storage_handler.get_data_stats_mut();

    assert!(stats.is_some(), "Failed to read from the storage stats (stats is None)");

    let stats_unwraped = stats.unwrap();

    let stat_from_binary = &stats_unwraped[0];

    assert_eq!(stat_from_binary.get_average_wpm(), stat.get_average_wpm());
  }
}
