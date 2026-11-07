use super::stat_helper::StatHelper;
use crate::config::{PracticeSection, TypingDuration};
use ratatui::{
  style::{Color, Style},
  text::{Line, Span},
};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Stat {
  typing_duration: TypingDuration,

  practice_section: PracticeSection,

  average_wpm: usize,

  raw_wpm: usize,

  accuracy: f64,

  true_accuracy: f64,

  timestamp_secs: u64,
}

impl Stat {
  /// Creates a new Stat
  ///
  /// Calculates the:
  ///   * WPM
  ///   * Raw WPM
  ///   * Accuracy
  pub fn new(
    typing_duration: TypingDuration,
    chars_counter: usize,
    mistakes_counter: usize,
    true_mistakes_counter: usize,
    practice_section: PracticeSection,
  ) -> Self {
    let typing_duration_in_seconds = typing_duration.as_seconds();

    let timestamp_secs = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .map(|duration| duration.as_secs())
      .unwrap_or(0);

    Self {
      typing_duration: typing_duration.clone(),
      practice_section,
      average_wpm: StatHelper::get_calculated_wpm(
        chars_counter,
        mistakes_counter,
        typing_duration_in_seconds,
      ),
      raw_wpm: StatHelper::get_calculated_raw_wpm(chars_counter, typing_duration_in_seconds),
      accuracy: StatHelper::get_calculated_accuracy(chars_counter, mistakes_counter),
      true_accuracy: StatHelper::get_calculated_accuracy(chars_counter, true_mistakes_counter),
      timestamp_secs,
    }
  }

  /// Returns the average wpm
  pub fn get_average_wpm(&self) -> usize {
    self.average_wpm
  }

  /// Returns the practice section of the run
  #[allow(unused)]
  pub fn get_practice_section(&self) -> PracticeSection {
    self.practice_section
  }

  /// Returns the practice section as a span
  pub fn get_section_pretty(&self) -> Span<'static> {
    match self.practice_section {
      PracticeSection::Languages => Span::from("Language"),
      PracticeSection::Code => Span::from("Code"),
    }
  }

  /// Returns the run date as "DD Mon YY"
  ///
  /// E.g. `07 Nov 26`
  pub fn get_date_pretty(&self) -> String {
    StatHelper::format_run_date(self.timestamp_secs)
  }

  /// Returns the run timestamp (seconds since the Unix epoch)
  #[allow(unused)]
  pub fn get_timestamp_secs(&self) -> u64 {
    self.timestamp_secs
  }

  /// Returns the duration
  pub fn get_time_difficulty(&self) -> Span<'static> {
    match self.typing_duration {
      TypingDuration::FifteenSec => {
        Span::from(" (Super short)").style(Style::default().fg(Color::Cyan))
      }
      TypingDuration::ThirtySec => Span::from(" (Short)").style(Style::default().fg(Color::Green)),
      TypingDuration::Minute => Span::from(" (Medium)").style(Style::default().fg(Color::Yellow)),
      TypingDuration::ThreeMinutes => Span::from(" (Long)").style(Style::default().fg(Color::Red)),
    }
  }

  /// Returns the duration
  pub fn get_duration_pretty(&self) -> Line<'static> {
    Line::from(vec![
      Span::from(format!("{}s", self.typing_duration.as_seconds())),
      self.get_time_difficulty(),
    ])
  }

  /// Returns the raw WPM
  pub fn get_raw_wpm(&self) -> usize {
    self.raw_wpm
  }

  /// Returns the accuracy
  pub fn get_accuracy(&self) -> f64 {
    self.accuracy
  }

  pub fn get_true_accuracy(&self) -> f64 {
    self.true_accuracy
  }
}
