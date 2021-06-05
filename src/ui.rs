use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::Color;
use tui::widgets::{Block, Borders, canvas::Canvas};
use tui::widgets::canvas::{Map, MapResolution};

use crate::gamestate::GameState;

pub fn draw<B: Backend>(f: &mut Frame<B>, state: &GameState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10)
        ].as_ref())
        .split(f.size());

    let midsection = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(80)
        ].as_ref())
        .split(chunks[1]);

    let top_block = Block::default()
        .title("Top block!")
        .borders(Borders::ALL);
    f.render_widget(top_block, chunks[0]);

    let sidebar = Block::default()
        .title("Sidebar!")
        .borders(Borders::ALL);
    f.render_widget(sidebar, midsection[0]);

    let main = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("World"))
        .paint(|ctx| {
            ctx.draw(&Map {
                color: Color::White,
                resolution: MapResolution::High,
            });
            ctx.print(state.player_x as f64, state.player_y as f64, "You are here", Color::Yellow);
        })
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0]);
    f.render_widget(main, midsection[1]);

    let commandbar = Block::default()
        .title("Log")
        .borders(Borders::ALL);
    f.render_widget(commandbar, chunks[2]);
}