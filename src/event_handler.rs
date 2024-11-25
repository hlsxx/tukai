use std::{error, io, time::Duration};

use ratatui::crossterm::event::{Event, KeyEvent, EventStream};
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
  _task: tokio::task::JoinHandle<()>,
}

impl EventHandler {
  pub fn new() -> Self {
    let tick_rate = Duration::from_secs(1);
    let (_tx, rx) = mpsc::unbounded_channel::<TukaiEvent>();

    let tx_clone = _tx.clone();

    let _task = tokio::spawn(async move {
      let mut reader = EventStream::new();
      let mut interval = tokio::time::interval(tick_rate);

      loop {
        let tick_delay = interval.tick();
        let crossterm_event = reader.next().fuse();

        tokio::select! {
          Some(Ok(event)) = crossterm_event => {
            match event {
              Event::Key(key_event) => {
                tx_clone.send(TukaiEvent::Key(key_event)).unwrap()
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
      rx,
      _task
    }
  }

  pub async fn next(&mut self) -> Result<TukaiEvent, Box<dyn error::Error>> {
    self.rx.recv().await.ok_or(Box::new(
      io::Error::new(io::ErrorKind::Other, "Some IO error occured")))
  }
}

pub struct PlatformApi;

impl PlatformApi {
  #[allow(inactive_code)]
  #[cfg(target_os = "windows")]
  fn is_capslock_on() -> bool {
    use winapi::um::winuser::{GetKeyState, VK_CAPITAL};
    unsafe { GetKeyState(VK_CAPITAL) & 0x0001 != 0 }
  }

  #[cfg(target_os = "linux")]
  #[allow(unused)]
  fn is_capslock_on_wayland() -> bool {
    false
  }

  #[cfg(target_os = "linux")]
  fn is_capslock_on_x11() -> bool {
    use x11::xlib::{XCloseDisplay, XOpenDisplay, XkbGetIndicatorState};
    use std::ptr;

    let display = unsafe { XOpenDisplay(ptr::null()) };

    if display.is_null() {
      eprintln!("Unable to open X display");
      return false;
    }

    let mut state: u32 = 0;
    let result = unsafe { XkbGetIndicatorState(display, 0x0100, &mut state) };

    unsafe { XCloseDisplay(display) };

    result == 0 && state & 0x01 != 0
  }

  #[cfg(target_os = "linux")]
  pub fn is_capslock_on() -> bool {
    use std::env;

    if env::var("DISPLAY").is_ok() {
      return PlatformApi::is_capslock_on_x11();
    }

    false
  }
}
