use crossterm::event::{self, Event, KeyEvent};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub struct EventHandler {
    rx: mpsc::Receiver<KeyEvent>,
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || loop {
            if event::poll(tick_rate).unwrap_or(false) {
                if let Event::Key(key) = event::read().unwrap_or(Event::FocusGained) {
                    let _ = tx.send(key);
                }
            }
        });
        EventHandler { rx }
    }

    pub fn next(&self) -> std::io::Result<KeyEvent> {
        self.rx
            .recv()
            .map_err(|_| std::io::Error::other("Event channel closed"))
    }
}
