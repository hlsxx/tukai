pub mod typing_window_config;
pub mod app_config;

#[allow(unused)]
pub trait ConfigBuilder<T> {
  fn new() -> Self;
  fn build(self) -> T;
}
