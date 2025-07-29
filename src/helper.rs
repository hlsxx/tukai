use std::iter::{repeat, repeat_n};

use crate::config::TukaiConfig;
use rand::{seq::SliceRandom, Rng};

pub struct Generator;

impl Generator {
  /// Loads a list of words from the current language from the configuration.
  ///
  /// This method attempts to read words from the language specified
  /// in the provided [`TukaiConfig`]. If the word list cannot be readed
  /// (e.g., the file is missing or unreadable), it returns an empty vector.
  pub fn get_words(config: &TukaiConfig) -> Vec<String> {
    config
      .get_language()
      .load_language_words()
      .unwrap_or(Vec::new())
  }

  /// Generates a random string composed of words from a language-specific word list.
  ///
  /// This method reads words from a `words/{language}.txt` file based on the language
  /// setting in the provided [`TukaiConfig`] and returns a randomly generated string.
  ///
  /// # Parameters
  /// - `config`: A reference to a [`TukaiConfig`] instance containing configuration options,
  ///   including the language to use.
  ///
  /// # Returns
  /// A `String` composed of randomly selected words.
  pub fn generate_random_string(config: &TukaiConfig) -> String {
    let mut rng = rand::thread_rng();

    Generator::get_words(config)
      .choose_multiple(&mut rng, config.typing_duration.as_seconds() * 2)
      .fold(String::new(), |mut acc, c| {
        acc.push_str(format!("{} ", c).as_str());
        acc
      })
  }

  /// Generates a repeated word string based on the provided configuration.
  ///
  /// The word and the number of repetitions are taken from the [`TukaiConfig`] instance.
  /// This is useful for producing visual effects like emphasis or animation in text-based UIs.
  ///
  /// # Parameters
  /// - `config`: A reference to a [`TukaiConfig`] instance containing configuration options,
  ///   including the language to use.
  ///
  /// # Returns
  /// A `String` composed of randomly selected multiple times repeated word.
  pub fn generate_repeated_word(config: &TukaiConfig) -> String {
    let mut rng = rand::thread_rng();
    let word = Generator::get_words(config)
      .choose(&mut rng)
      .cloned()
      .unwrap_or(String::from("Hello"));

    repeat_n(word, 50).collect::<Vec<String>>().join(" ")
  }

  /// Generates and returns a random motto string.
  ///
  /// This could be used, for example, in a screen footer.
  /// # Returns
  /// A random motto `String` selected from a predefined list of mottos.
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
