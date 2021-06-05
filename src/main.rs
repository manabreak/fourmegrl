use std::{io, thread};
use std::io::Stdout;
use std::sync::mpsc;
use std::thread::sleep;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    execute,
};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen};
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::Terminal;
use tui::widgets::{Block, Borders};

use crate::gamestate::GameState;

mod ui;
mod gamestate;

enum Event<I> {
    Input(I)
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode().unwrap();

    let mut stdout = io::stdout();
    execute!(stdout, crossterm::terminal::EnterAlternateScreen);

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = Duration::from_millis(250)
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if event::poll(timeout).unwrap() {
                if let CEvent::Key(key) = event::read().unwrap() {
                    tx.send(Event::Input(key)).unwrap();
                }
            } else {
                last_tick = Instant::now();
            }
        }
    });

    terminal.clear().unwrap();

    let mut state = GameState::init();

    loop {
        terminal.draw(|f| ui::draw(f, &state)).unwrap();
        match rx.recv().unwrap() {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode().unwrap();
                    execute!(terminal.backend_mut(), LeaveAlternateScreen).unwrap();
                    break;
                }
                _ => {}
            },
            _ => {}
        }
    }

    Ok(())
}