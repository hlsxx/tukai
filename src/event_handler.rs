use std::{error, io, time::Duration};

use crossterm::event::{EventStream, KeyEvent, Event};
use tokio::sync::mpsc;
use futures::{FutureExt, StreamExt};

#[derive(Clone, Copy, Debug)]
pub enum TukajEvent {
  Tick,
  Key(KeyEvent),
}

pub struct EventHandler {
  tx: mpsc::UnboundedSender<TukajEvent>,
  rx: mpsc::UnboundedReceiver<TukajEvent>,
  task: tokio::task::JoinHandle<()>,
}

impl EventHandler {
  pub fn new() -> Self {
    let tick_rate = Duration::from_secs(1);
    let (tx, rx) = mpsc::unbounded_channel::<TukajEvent>();

    let tx_clone = tx.clone();

    let task = tokio::spawn(async move {
      let mut reader = EventStream::new();
      let mut interval = tokio::time::interval(tick_rate);

      loop {
        let tick_delay = interval.tick();
        let crossterm_event = reader.next().fuse();

        tokio::select! {
          Some(Ok(event)) = crossterm_event => {
            match event {
              Event::Key(key_event) => {
                tx_clone.send(TukajEvent::Key(key_event)).unwrap()
              },
              _ => {}
            }
          },
          _ = tick_delay => {
            tx_clone.send(TukajEvent::Tick).unwrap();
          },
        }
      }
    });

    Self {
      tx,
      rx,
      task
    }
  }

  pub async fn next(&mut self) -> Result<TukajEvent, Box<dyn error::Error>> {
    self.rx.recv().await.ok_or(Box::new(
      io::Error::new(io::ErrorKind::Other, "Some IO error occured")))
  }
}
