use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use super::helpers::stat_helper::StatHelper;

pub type Stats = HashMap<StatDuration, Vec<Stat>>;

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
