use std::iter::repeat;

use crate::config::TukaiConfig;
use rand::{seq::SliceRandom, Rng};

pub struct Generator;

impl Generator {
  pub fn get_words(config: &TukaiConfig) -> Vec<String> {
    config
      .get_language()
      .load_language_words()
      .unwrap_or(Vec::new())
  }

  /// Generates a random string of words.
  ///
  /// This method generates a string containing random
  /// words from the words/{language}.txt file
  pub fn generate_random_string(config: &TukaiConfig) -> String {
    let mut rng = rand::thread_rng();

    Generator::get_words(config)
      .choose_multiple(&mut rng, config.typing_duration.as_seconds() * 2)
      .fold(String::new(), |mut acc, c| {
        acc.push_str(format!("{} ", c).as_str());
        acc
      })
  }

  pub fn generate_repeated_word(config: &TukaiConfig) -> String {
    let mut rng = rand::thread_rng();
    let word = Generator::get_words(config)
      .choose(&mut rng)
      .cloned()
      .unwrap_or(String::from("Hello"));

    let generated_string = repeat(word).take(50).collect::<Vec<String>>().join(" ");

    generated_string
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
