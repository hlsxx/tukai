use serde::{Deserialize, Serialize};
use crate::configs::ConfigBuilder;

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Debug, Clone)]
/// Represents the available durations for the test
///
/// This enum defines default durations
///
/// # Variants
/// - `ThirtySec` - 30 seconds typing duration
/// - `Minute` - 60 seconds typing duration
/// - `ThreeMinutes` - 180 seconds typing duration
pub enum TypingDuration {
  ThirtySec,
  Minute,
  ThreeMinutes
}

impl Default for TypingDuration {
  fn default() -> Self {
    Self::Minute
  }
}

impl TypingDuration {
  // pub fn to_string(&self) -> String {
  //   let time = match self {
  //     TypingDuration::Minute => "60s",
  //     TypingDuration::ThirtySec => "30s",
  //     TypingDuration::ThreeMinutes => "180s"
  //   };
  //
  //   time.to_string()
  // }

  pub fn as_seconds(&self) -> usize {
    match self {
      TypingDuration::ThirtySec => 30,
      TypingDuration::Minute => 60,
      TypingDuration::ThreeMinutes => 180
    }
  }
}

pub struct TypingWindowConfig {
  pub typing_duration: TypingDuration,
}

impl Default for TypingWindowConfig {
  fn default() -> Self {
    Self {
      typing_duration: TypingDuration::default()
    }
  }
}

impl TypingWindowConfig {
  /// Switch the typing duration
  ///
  /// ThirtySec
  /// Minute
  /// ThreeMinutes
  pub fn switch_typing_duration(&mut self) {
    if self.typing_duration == TypingDuration::Minute {
      self.typing_duration = TypingDuration::ThreeMinutes;
    } else if self.typing_duration == TypingDuration::ThreeMinutes {
      self.typing_duration = TypingDuration::ThirtySec
    } else if self.typing_duration == TypingDuration::ThirtySec {
      self.typing_duration = TypingDuration::Minute
    }
  }
}

pub struct TypingWindowConfigBuilder {
  typing_duration: Option<TypingDuration>
}

impl TypingWindowConfigBuilder {
  #[allow(unused)]
  fn time_limit(mut self, typing_duration: TypingDuration) -> Self {
    self.typing_duration = Some(typing_duration);
    self
  }
}

impl ConfigBuilder<TypingWindowConfig> for TypingWindowConfigBuilder {
  fn new() -> Self {
    Self {
      typing_duration: None
    }
  }

  fn build(self) -> TypingWindowConfig {
    let typing_window_config = TypingWindowConfig::default();

    TypingWindowConfig {
      typing_duration: self.typing_duration.unwrap_or(typing_window_config.typing_duration)
    }
  }
}


