use std::collections::VecDeque;

use ratatui::layout::{Position, Rect};

struct SpacePosition {
  near_space_index: usize,
  indexes: VecDeque<usize>
}

impl SpacePosition {
  pub fn new(text: &str) -> Self {
    let mut indexes = text
      .chars()
      .enumerate()
      .filter(|(_, c)| *c == ' ')
      .map(|(i, _)| i)
      .collect::<VecDeque<usize>>();

    Self { near_space_index: indexes.pop_front().unwrap_or(0), indexes }
  }

  pub fn near_space_index(&self) -> usize {
    self.near_space_index
  }

  pub fn next_index(&mut self) {
    self.near_space_index = self.indexes.pop_front().unwrap_or(0);
  }
}

pub struct Cursor {
  space_position: SpacePosition,

  /// Index within a generated text
  pub index: u16,

  // X axis position of the cursor
  x: u16,

  // Y axis position of the cursor
  y: u16,
}

impl Cursor {
  pub fn new(text: &str) -> Self {
    Self {
      space_position: SpacePosition::new(text),
      index: 0,
      x: 0,
      y: 0
    }
  }

  pub fn is_space(&mut self, index: usize) -> bool {
    if index == self.space_position.near_space_index() {
      self.space_position.next_index();
      true
    } else {
      false
    }
  }

  pub fn reset(&mut self) {
    self.index = 0;
    self.x = 0;
    self.y = 0;
  }

  pub fn move_forward(&mut self) {
    self.index += 1;
  }

  pub fn move_backward(&mut self) {
    self.index -= 1;
  }

  pub fn index(&self) -> usize {
    self.index as usize
  }

  /// Returns absolute position of the cursor.
  ///
  /// Contains both paddings and total `area.height`.
  pub fn prepare_absolute_position(&self, area: &Rect) -> [u16; 2] {
    let left_padding = 40;
    let top_padding = (area.height / 2) - 5;

    let x = area.x + left_padding + 1;
    let y = area.y + top_padding + 3;

    [x, y]
  }

  pub fn positition(&mut self, area: &Rect) -> Position {
    let max_width = area.width / 2;

    self.y = self.index / max_width;
    self.x = self.index % max_width;

    let [cursor_x, cursor_y] = self.prepare_absolute_position(area);
    Position::new(cursor_x + self.x, cursor_y + self.y)
  }
}
