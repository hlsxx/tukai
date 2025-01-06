use std::{error, io, time::Duration};

use ratatui::crossterm::event::{Event, KeyEvent, EventStream};
use tokio::sync::mpsc;

#[cfg(target_os = "windows")]
use tokio::time::Instant;

use futures::{FutureExt, StreamExt};

#[derive(Clone, Copy, Debug)]
pub enum TukaiEvent {
  Tick,
  Key(KeyEvent),
}

pub struct EventHandler {
  _tx: mpsc::UnboundedSender<TukaiEvent>,
  rx: mpsc::UnboundedReceiver<TukaiEvent>
}

impl EventHandler {
  pub fn new() -> Self {
    let tick_rate = Duration::from_secs(1);
    let (_tx, rx) = mpsc::unbounded_channel::<TukaiEvent>();

    let tx_clone = _tx.clone();

    tokio::spawn(async move {
      let mut reader = EventStream::new();
      let mut interval = tokio::time::interval(tick_rate);

      // Debounce input events
      #[cfg(target_os = "windows")]
      let mut last_key_event: Option<(KeyEvent, Instant)> = None;

      #[cfg(target_os = "windows")]
      let debounce_duration = Duration::from_millis(50);

      loop {
        let tick_delay = interval.tick();
        let crossterm_event = reader.next().fuse();

        tokio::select! {
          Some(Ok(event)) = crossterm_event => {
            match event {
              Event::Key(key_event) => {
                #[cfg(target_os = "windows")]
                {
                  let now = Instant::now();

                  let is_duplicate = last_key_event
                    .as_ref()
                    .map(|(prev_event, prev_time)| {
                      *prev_event == key_event && now.duration_since(*prev_time) < debounce_duration
                    })
                    .unwrap_or(false);
                  
                  if is_duplicate { continue }

                  last_key_event = Some((key_event, now));
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

    Self {
      _tx,
      rx
    }
  }

  pub async fn next(&mut self) -> Result<TukaiEvent, Box<dyn error::Error>> {
    self.rx.recv().await.ok_or(Box::new(
      io::Error::new(io::ErrorKind::Other, "Some IO error occured")))
  }
}
