use std::{collections::HashMap, path::{Path, PathBuf}};
use serde::{Deserialize, Serialize};

use crate::file_handler::FileHandler;

#[derive(Serialize, Deserialize, Debug)]
pub struct Stat {
  average_wpm: usize,
  raw_wpm: usize,
  accuracy: f32
}

impl Stat {
  pub fn new(
    chars_counter: usize,
    mistakes_counter: usize,
    time_limit: usize
  ) -> Self {
    Self {
      average_wpm: StatHelper::get_calculated_wpm(chars_counter, mistakes_counter, time_limit),
      raw_wpm: StatHelper::get_calculated_raw_wpm(chars_counter, time_limit),
      accuracy: StatHelper::get_calculated_accuracy(chars_counter, mistakes_counter)
    }
  }
}

struct StatHelper {}

impl StatHelper {

  /// Calculates raw WPM
  pub fn get_calculated_raw_wpm(
    chars_counter: usize,
    time_limit: usize
  ) -> usize {
    (chars_counter / 5) * 60 / time_limit as usize
  }

  /// Calculates WPM
  pub fn get_calculated_wpm(
    chars_counter: usize,
    mistakes_counter: usize,
    time_limit: usize
  ) -> usize {
    (chars_counter.saturating_sub(mistakes_counter) / 5) * 60 / time_limit as usize
  }

  /// Calculates accuracy
  pub fn get_calculated_accuracy(
    chars_counter: usize,
    mistakes_counter: usize
  ) -> f32 {
    let accuracy = (chars_counter.saturating_sub(mistakes_counter * 100)) as f32 / chars_counter as f32;
    (accuracy * 100.0).round() / 100.0
  }

}

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Debug)]
enum StatDuration {
  ThirtySec,
  Minute,
  ThreeMinutes
}

#[allow(unused)]
impl StatDuration {
  pub fn as_seconds(&self) -> u32 {
    match self {
      StatDuration::ThirtySec => 30,
      StatDuration::Minute => 60,
      StatDuration::ThreeMinutes => 180
    }
  }
}

type Stats = HashMap<StatDuration, Vec<Stat>>;
type Activities = HashMap<String, String>;

enum StorageDataType {
  Stats,
  Activities
}

enum StorageDataValue {
  Stats(Stats),
  Activity(Activities)
}

pub struct Storage {
  file_path: PathBuf,
  data: HashMap<StorageDataType, StorageDataValue>,
}

impl Storage {
  pub fn new<P: AsRef<Path>>(file_path: P) -> Self {
    Self {
      file_path: file_path.as_ref().to_owned(),
      data: HashMap::new()
    }
  }

  pub fn insert_stat(
    &mut self,
    stat: &Stat
  ) -> Result<(), std::io::Error> {
    let stat_bytes = bincode::serialize(stat).unwrap();
    FileHandler::write_bytes_into_file(&self.file_path, &stat_bytes)?;

    Ok(())
  }

  pub fn read_from_stats(file_path: &Path) -> Result<(), std::io::Error> {
    let stats_bytes = FileHandler::read_bytes_from_file(file_path)?;

    let stats = bincode::deserialize::<Stats>(&stats_bytes).unwrap();

    println!("{:?}", stats);

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn app_data_insert_read_from_file() {
    let stat = Stat {
      average_wpm: 80,
      raw_wpm: 90,
      accuracy: 95.50
    };

    // AppData::insert_into_run_stats(&run_stat).expect("Error")
  }
}
