pub struct StatHelper;

impl StatHelper {
  /// Calculates raw WPM
  pub fn get_calculated_raw_wpm(chars_counter: usize, time_limit: usize) -> usize {
    ((chars_counter as f64 / 5.0) * 60.0 / time_limit as f64) as usize
  }

  /// Calculates WPM
  pub fn get_calculated_wpm(
    chars_counter: usize,
    mistakes_counter: usize,
    time_limit: usize,
  ) -> usize {
    (((chars_counter as f64 - mistakes_counter as f64) / 5.0) * 60.0 / time_limit as f64).round()
      as usize
  }

  /// Calculates accuracy
  pub fn get_calculated_accuracy(chars_counter: usize, mistakes_counter: usize) -> f64 {
    let accuracy =
      ((chars_counter as f64 - mistakes_counter as f64) / chars_counter as f64) * 100.0;
    (accuracy * 100.0).round() / 100.0
  }
}
