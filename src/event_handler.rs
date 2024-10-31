use std::time::Duration;

use crossterm::event::{EventStream, KeyEvent};
use tokio::{sync::mpsc, task::JoinHandle};

#[derive(Clone, Copy, Debug)]
pub enum Event {
  Error,
  Tick,
  Key(KeyEvent),
}

struct EventHandler {
  _tx: mpsc::UnboundedSender<Event>,
  rx: mpsc::UnboundedReceiver<Event>,
  task: Option<JoinHandle<()>>,
}

impl EventHandler {
  fn new() -> Self {
    let tick_rate = Duration::from_millis(250);

    let (tx, rx) = mpsc::unbounded_channel();

    let _tx = tx.clone();

    let task = tokio::spawn(async move {
      let mut reader = EventStream::new();
      let mut interval = tokio::time::interval(tick_rate);

      loop {
        interval.tick().await;
      }
    });

    Self {
      _tx,
      rx,
      task: Some(task)
    }
  }
}
