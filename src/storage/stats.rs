use super::stat_helper::StatHelper;
use crate::config::TypingDuration;
use ratatui::{
  style::{Color, Style},
  text::{Line, Span},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Stat {
  typing_duration: TypingDuration,

  average_wpm: usize,

  raw_wpm: usize,

  accuracy: f64,
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
  ) -> Self {
    let typing_duration_in_seconds = typing_duration.as_seconds();

    Self {
      typing_duration: typing_duration.clone(),
      average_wpm: StatHelper::get_calculated_wpm(
        chars_counter,
        mistakes_counter,
        typing_duration_in_seconds,
      ),
      raw_wpm: StatHelper::get_calculated_raw_wpm(chars_counter, typing_duration_in_seconds),
      accuracy: StatHelper::get_calculated_accuracy(chars_counter, mistakes_counter),
    }
  }

  /// Returns the average wpm
  pub fn get_average_wpm(&self) -> usize {
    self.average_wpm
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
      Span::from(format!(
        "{}s",
        self.typing_duration.as_seconds().to_string()
      )),
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
}
