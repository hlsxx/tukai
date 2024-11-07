use std::{collections::HashMap, path::PathBuf};
use serde::{Deserialize, Serialize};

use crate::file_handler::FileHandler;

#[derive(Serialize, Deserialize)]
struct RunStat {
  average_wpm: usize,
  raw_wpm: usize,
  accuracy: f32
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

struct AppData {
  run_stats: HashMap<RunStatDuration, Vec<RunStat>>
}

impl AppData {
  pub fn insert_into_run_stats(run_stat: &RunStat) -> Result<(), std::io::Error> {
    let run_stat_bytes = bincode::serialize(run_stat).unwrap();

    FileHandler::write_bytes_into_file(PathBuf::from("xxx.tukai"), &run_stat_bytes)?;

    println!("{:?}", FileHandler::read_bytes_from_file(PathBuf::from("xxx.tukai"))?);

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn app_data_insert_into_run_stats_test_one() {
    let run_stat = RunStat {
      average_wpm: 80,
      raw_wpm: 90,
      accuracy: 95.50
    };

    AppData::insert_into_run_stats(&run_stat).unwrap()
  }
}
