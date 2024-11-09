pub struct StatHelper {}

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
