use std::{error, io, time::Duration};

use ratatui::crossterm::event::{Event, EventStream, KeyEvent};

#[cfg(target_os = "windows")]
use ratatui::crossterm::event::KeyEventKind;

use tokio::sync::mpsc;

use futures::{FutureExt, StreamExt};

#[derive(Clone, Copy, Debug)]
pub enum TukaiEvent {
  Tick,
  Key(KeyEvent),
}

pub struct EventHandler {
  _tx: mpsc::UnboundedSender<TukaiEvent>,
  rx: mpsc::UnboundedReceiver<TukaiEvent>,
}

impl EventHandler {
  pub fn new() -> Self {
    let tick_rate = Duration::from_secs(1);
    let (_tx, rx) = mpsc::unbounded_channel::<TukaiEvent>();

    let tx_clone = _tx.clone();

    tokio::spawn(async move {
      let mut reader = EventStream::new();
      let mut interval = tokio::time::interval(tick_rate);

      loop {
        let tick_delay = interval.tick();
        let crossterm_event = reader.next().fuse();

        tokio::select! {
          Some(Ok(event)) = crossterm_event => {
            match event {
              Event::Key(key_event) => {

                // On Windows terminal takes press and release
                // To avoid symbols duplications checks a event kind
                #[cfg(target_os = "windows")]
                {
                  if key_event.kind != KeyEventKind::Press {
                    continue
                  }
                }

                tx_clone.send(TukaiEvent::Key(key_event)).unwrap();
              },
              _ => {}
            }
          },
          _ = tick_delay => {
            tx_clone.send(TukaiEvent::Tick).unwrap();
          },
        }
      }
    });

    Self { _tx, rx }
  }

  pub async fn next(&mut self) -> Result<TukaiEvent, Box<dyn error::Error>> {
    self.rx.recv().await.ok_or(Box::new(io::Error::new(
      io::ErrorKind::Other,
      "Some IO error occured",
    )))
  }
}
