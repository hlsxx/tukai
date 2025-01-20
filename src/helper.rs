use rand::{seq::SliceRandom, Rng};
use crate::config::TukaiConfig;

pub struct Generator;

impl Generator {

  /// Generates a random string of words
  ///
  /// This method generates a string containing random
  /// words from the words/{language}.txt file
  pub fn generate_random_string(config: &TukaiConfig) -> String {
    // Tries to load a language dictionary or creates empty vec
    let words = config
      .get_language()
      .load_language_words()
      .unwrap_or(Vec::new());

    let mut rng = rand::thread_rng();

    let text = words
      .choose_multiple(&mut rng, config.typing_duration.as_seconds() * 2)
      .fold(String::new(), |mut acc, c| {
        acc.push_str(format!("{} ", c).as_str());
        acc
      });

    text
  }

  /// Generates a random motto for the block bottom title.
  pub fn generate_random_motto() -> String {
    let mottos = [
      " Practice today, master tomorrow ",
      " Fingers on keys, progress with ease ",
      " Consistency breeds accuracy ",
      " Type smarter, not harder ",
      " Precision today, perfection tomorrow ",
    ];

    let mut rng = rand::thread_rng();

    let random_index = rng.gen_range(0..mottos.len());

    String::from(mottos[random_index])
  }
}
