use serde::{Deserialize, Serialize};
use super::stat_helper::StatHelper;

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Debug, Clone)]
pub enum TypingDuration {
  ThirtySec,
  Minute,
  ThreeMinutes
}

impl TypingDuration {
  pub fn to_string(&self) -> String {
    let time = match self {
      TypingDuration::Minute => "60s",
      TypingDuration::ThirtySec => "30s",
      TypingDuration::ThreeMinutes => "180s"
    };

    time.to_string()
  }
}

#[allow(unused)]
impl TypingDuration {
  pub fn as_seconds(&self) -> u32 {
    match self {
      TypingDuration::ThirtySec => 30,
      TypingDuration::Minute => 60,
      TypingDuration::ThreeMinutes => 180
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stat {
  duration: TypingDuration,
  average_wpm: usize,
  raw_wpm: usize,
  accuracy: f32
}

impl Stat {
  pub fn new(
    duration: TypingDuration,
    chars_counter: usize,
    mistakes_counter: usize,
    time_limit: usize
  ) -> Self {
    Self {
      duration,
      average_wpm: StatHelper::get_calculated_wpm(chars_counter, mistakes_counter, time_limit),
      raw_wpm: StatHelper::get_calculated_raw_wpm(chars_counter, time_limit),
      accuracy: StatHelper::get_calculated_accuracy(chars_counter, mistakes_counter)
    }
  }

  pub fn get_duration(&self) -> &TypingDuration {
    &self.duration
  }

  pub fn get_average_wpm(&self) -> usize {
    self.average_wpm
  }

  pub fn get_raw_wpm(&self) -> usize {
    self.raw_wpm
  }

  pub fn get_accuracy(&self) -> f32 {
    self.accuracy
  }

}
