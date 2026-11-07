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

  /// Formats a Unix timestamp (seconds) as "DD Mon YY"
  ///
  /// E.g. `07 Nov 26`
  pub fn format_run_date(timestamp_secs: u64) -> String {
    let days = (timestamp_secs / 86_400) as i64;

    let (year, month, day) = Self::civil_from_days(days);

    format!("{day:02} {} {:02}", Self::month_name(month), year % 100)
  }

  /// Converts days since the Unix epoch into a civil date
  ///
  /// Uses the days-from-civil algorithm by Howard Hinnant
  fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;

    let era = z / 146_097;
    let day_of_era = z - era * 146_097;
    let year_of_era = (day_of_era - day_of_era / 1_460 + day_of_era / 36_524
      - day_of_era / 146_096)
      / 365;

    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_index = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_index + 2) / 5 + 1;
    let month = if month_index < 10 {
      month_index + 3
    } else {
      month_index - 9
    };

    ((if month <= 2 { year + 1 } else { year }), month as u32, day as u32)
  }

  /// Returns the three first letters of the month name
  fn month_name(month: u32) -> &'static str {
    const MONTH_NAMES: [&str; 12] = [
      "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];

    MONTH_NAMES
      .get(month as usize - 1)
      .copied()
      .unwrap_or("???")
  }
}

#[cfg(test)]
mod tests {
  use super::StatHelper;

  #[test]
  fn formats_epoch() {
    // 1970-01-01 00:00:00 UTC
    assert_eq!(StatHelper::format_run_date(0), "01 Jan 70");
  }

  #[test]
  fn formats_leap_day() {
    // 2024-02-29 00:00:00 UTC
    assert_eq!(StatHelper::format_run_date(1_709_164_800), "29 Feb 24");
  }

  #[test]
  fn formats_year_boundary() {
    // 2023-12-31 and 2024-01-01 00:00:00 UTC
    assert_eq!(StatHelper::format_run_date(1_703_980_800), "31 Dec 23");
    assert_eq!(StatHelper::format_run_date(1_704_067_200), "01 Jan 24");
  }

  #[test]
  fn formats_recent_date() {
    // 2026-11-07 00:00:00 UTC
    assert_eq!(StatHelper::format_run_date(1_794_009_600), "07 Nov 26");
  }
}
