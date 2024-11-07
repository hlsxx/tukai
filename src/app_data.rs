use std::{collections::HashMap, path::{Path, PathBuf}};
use serde::{Deserialize, Serialize};

use crate::file_handler::FileHandler;

#[derive(Serialize, Deserialize)]
pub struct RunStat {
  average_wpm: usize,
  raw_wpm: usize,
  accuracy: f32
}

impl RunStat {
  pub fn new(
    chars_counter: usize,
    mistakes_counter: usize,
    time_limit: usize
  ) -> Self {
    Self {
      average_wpm: RunStatHelper::get_calculated_wpm(chars_counter, mistakes_counter, time_limit),
      raw_wpm: RunStatHelper::get_calculated_raw_wpm(chars_counter, time_limit),
      accuracy: RunStatHelper::get_calculated_accuracy(chars_counter, mistakes_counter)
    }
  }
}

struct RunStatHelper {}

impl RunStatHelper {

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

enum RunStatDuration {
  ThirtySec,
  Minute,
  ThreeMinutes
}

#[allow(unused)]
impl RunStatDuration {
  pub fn as_seconds(&self) -> u32 {
    match self {
      RunStatDuration::ThirtySec => 30,
      RunStatDuration::Minute => 60,
      RunStatDuration::ThreeMinutes => 180
    }
  }
}

pub struct AppData {
  run_stats: HashMap<RunStatDuration, Vec<RunStat>>
}

impl AppData {
  pub fn insert_into_run_stats(
    file_path: &Path,
    run_stat: &RunStat
  ) -> Result<(), std::io::Error> {
    let run_stat_bytes = bincode::serialize(run_stat).unwrap();
    FileHandler::write_bytes_into_file(file_path, &run_stat_bytes)?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn app_data_insert_read_from_file() {
    let run_stat = RunStat {
      average_wpm: 80,
      raw_wpm: 90,
      accuracy: 95.50
    };

    AppData::insert_into_run_stats(&run_stat).expect("Error")
  }
}
