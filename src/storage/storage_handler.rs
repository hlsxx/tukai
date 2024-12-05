use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use maplit::hashmap;
use serde::{Deserialize, Serialize};

use crate::file_handler::FileHandler;
use crate::layout::LayoutName;

use super::stats::Stat;

pub type Activities = Vec<String>;

#[derive(Deserialize, Serialize, Hash, PartialEq, Eq, Debug)]
pub enum StorageDataType {
  Stats,
  Activities,
  Layout
}

#[derive(Deserialize, Serialize, Debug)]
pub enum StorageDataValue {
  Stats(Vec<Stat>),
  Activites(Activities),
  Layout(LayoutName)
}

type StorageData = HashMap<StorageDataType, StorageDataValue>;

pub struct StorageHandler {
  file_path: PathBuf,

  // Data stored in the binary file
  data: StorageData,
}

impl StorageHandler {

  pub fn new<P: AsRef<Path>>(file_path: P) -> Self {
    let local_dir_path = dirs::data_local_dir()
      .unwrap_or(PathBuf::from("/tmp"));

    let full_path = local_dir_path.join("tukai")
      .join(file_path);

    Self {
      file_path: full_path,
      data: HashMap::new()
    }
  }

  #[cfg(test)]
  pub fn delete_file(&self) -> Result<(), io::Error> {
    use std::fs;

    fs::remove_file(&self.file_path)?;

    Ok(())
  }

  #[allow(unused)]
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

    let default_stats = StorageDataValue::Stats(Vec::new());
    let default_activities= StorageDataValue::Activites(Vec::new());
    let default_layout = StorageDataValue::Layout(LayoutName::Neptune);

    empty_data.insert(StorageDataType::Stats, default_stats);
    empty_data.insert(StorageDataType::Activities, default_activities);
    empty_data.insert(StorageDataType::Layout, default_layout);

    let data_bytes = bincode::serialize(&empty_data).unwrap();
    FileHandler::write_bytes_into_file(&self.file_path, &data_bytes)?;

    Ok(self)
  }

  /// Inits empty data and write into the storage file
  fn init_empty_data(&mut self) -> Result<(), io::Error> {
    use StorageDataType::*;

    self.data = hashmap! {
      Stats => StorageDataValue::Stats(Vec::new()),
      Activities => StorageDataValue::Activites(Vec::new()),
      Layout => StorageDataValue::Layout(LayoutName::Neptune)
    };

    let data_bytes = bincode::serialize(&self.data).unwrap();
    FileHandler::write_bytes_into_file(&self.file_path, &data_bytes)?;

    Ok(())
  }

  /// Inits the storage
  ///
  /// Try to read all bytes from the storage file
  /// Then set into the data
  pub fn init(mut self) -> Result<Self, io::Error> {
    if !self.file_path.exists() {
      self.init_empty_data()?;
      return Ok(self);
    }

    let data_bytes = FileHandler::read_bytes_from_file(&self.file_path)?;

    match bincode::deserialize(&data_bytes) {
      Ok(data) => self.data = data,
      Err(_) => self.init_empty_data()?
    };

    Ok(self)
  }

  #[allow(unused)]
  pub fn get_data(&self) -> &StorageData {
    &self.data
  }

  pub fn get_data_for_chart(&self) -> (usize, Vec<(f64, f64)>) {
    if let Some(StorageDataValue::Stats(stats)) = self.data.get(&StorageDataType::Stats) {
      let mut best_wpm = 0_usize;

      let dataset = stats.iter().enumerate()
        .rev()
        .map(|(index, stat)| {
          let stat_wpm = stat.get_average_wpm();

          best_wpm = best_wpm.max(stat_wpm);

          (index as f64, stat_wpm as f64)
        }).collect::<Vec<(f64, f64)>>();

      (best_wpm, dataset)
    } else {
      (100, Vec::new())
    }
  }

  pub fn get_data_stats_reversed(&self) -> Option<Vec<Stat>> {
    if let Some(StorageDataValue::Stats(stats)) = self.data.get(&StorageDataType::Stats) {
      let stats_reversed = stats.iter()
        .rev()
        .map(|item| item.to_owned()).collect::<Vec<Stat>>();

      Some(stats_reversed)
    } else {
      None
    }
  }

  pub fn get_data_stats_bets(&self) -> Option<Vec<Stat>> {
    if let Some(StorageDataValue::Stats(stats)) = self.data.get(&StorageDataType::Stats) {
      let mut x = stats.clone();
      x.sort_by(|a, b| b.get_average_wpm().cmp(&a.get_average_wpm()));

      Some(x)
    } else {
      None
    }
  }

  pub fn get_active_layout_name(&self) -> Option<LayoutName> {
    if let Some(StorageDataValue::Layout(layout)) = self.data.get(&StorageDataType::Layout) {
      Some(layout.clone())
    } else {
      None
    }
  }

  /// Gets the stats from the storage
  fn get_data_stats_mut(&mut self) -> Option<&mut Vec<Stat>> {
    if let Some(StorageDataValue::Stats(stats)) = self.data.get_mut(&StorageDataType::Stats) {
      Some(stats)
    } else {
      None
    }
  }

  /// Gets the activities from the storage
  #[allow(unused)]
  fn get_data_activities_mut(&mut self) -> Option<&Activities> {
    if let Some(StorageDataValue::Activites(activities)) = self.data.get_mut(&StorageDataType::Activities) {
      Some(activities)
    } else {
      None
    }
  }

  /// Flush all data
  pub fn flush(&self) -> Result<(), std::io::Error> {
    let data_bytes = bincode::serialize(&self.data)
      .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    FileHandler::write_bytes_into_file(&self.file_path, &data_bytes)
  }

  pub fn insert_into_stats(
    &mut self,
    stat: &Stat
  ) -> bool {
    if let Some(stats) = self.get_data_stats_mut() {
      stats.push(stat.clone());
      return self.flush().is_ok();
    }

    false
  }

  pub fn switch_layout(
    &mut self,
    layout_name_changed: LayoutName
  ) -> bool {
    if let Some(StorageDataValue::Layout(layout_name)) = self.data.get_mut(&StorageDataType::Layout) {
      *layout_name = layout_name_changed;
      return true;
    }

    false
  }

}

#[cfg(test)]
mod tests {
  use crate::storage::stats::TypingDuration;
  use uuid::Uuid;
  use super::*;

  fn get_storage_handler() -> StorageHandler {
    let storage_helper = StorageHandler::new(format!("tests/{}.tukai", Uuid::new_v4()))
      .init()
      .expect("Failed to initialize storage file");

    storage_helper
  }

  fn get_test_stat() -> Stat {
    Stat::new(
      TypingDuration::Minute,
      80,
      5,
      60
    )
  }

  //#[test]
  // fn storage_local_dir_initialization() {
  //   let storage_handler = get_storage_handler();
  // }

  #[test]
  // Just validate if binary file was created right
  fn storage_load() {
    let storage_handler = get_storage_handler();
    let storage_data = storage_handler.get_data();

    assert!(storage_data.get(&StorageDataType::Stats).is_some(), "Stats not initialized successfully");
    assert!(storage_data.get(&StorageDataType::Activities).is_some(), "Activities not initialized successfully");
    assert!(storage_data.get(&StorageDataType::Layout).is_some(), "Layout not initialized successfully");

    storage_handler.delete_file().expect("Error occured while deleting file");
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

    assert!(storage_handler.insert_into_stats(&stat), "Insert into the storage error occured");

    let stats = storage_handler.get_data_stats_mut();

    assert!(stats.is_some(), "Failed to read from the storage stats (stats is None)");

    let stats_unwraped = stats.unwrap();

    let stat_from_memory = &stats_unwraped[0];

    assert_eq!(stat_from_memory.get_average_wpm(), stat.get_average_wpm());

    storage_handler.delete_file().expect("Error occured while deleting file");
  }

  #[test]
  fn flush_data() {
    let mut storage_handler = get_storage_handler();
    storage_handler.insert_into_stats(&get_test_stat());

    assert!(storage_handler.flush().is_ok());

    storage_handler.delete_file().expect("Error occured while deleting file");
  }

  #[test]
  fn load_flushed_data() {
    let mut storage_handler = get_storage_handler();
    storage_handler.insert_into_stats(&get_test_stat());

    println!("{:?}", storage_handler.get_data());

    let data = storage_handler.get_data();
    println!("{:?}", data);

    storage_handler.delete_file().expect("Error occured while deleting file");
  }
}
