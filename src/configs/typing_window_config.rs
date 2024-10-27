use crate::traits::ConfigBuilder;

pub struct TypingWindowConfig {
  pub time_limit: usize
}

impl Default for TypingWindowConfig {
  fn default() -> Self {
    Self {
      time_limit: 60
    }
  }
}

pub struct TypingWindowConfigBuilder {
  time_limit: Option<usize>
}

impl TypingWindowConfigBuilder {
  #[allow(unused)]
  fn time_limit(mut self, time_limit: usize) -> Self {
    self.time_limit = Some(time_limit);
    self
  }
}

impl ConfigBuilder<TypingWindowConfig> for TypingWindowConfigBuilder {
  fn new() -> Self {
    Self {
      time_limit: None
    }
  }

  fn build(self) -> TypingWindowConfig {
    let typing_window_config = TypingWindowConfig::default();

    TypingWindowConfig {
      time_limit: self.time_limit.unwrap_or(typing_window_config.time_limit)
    }
  }

}


