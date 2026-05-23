mod api;
mod app;
mod input;
mod terminal;
mod ui;

use std::{io::Stdout, time::Duration};

use anyhow::Result;
use app::App;
use crossterm::event::{self, Event};
use ratatui::{Terminal, backend::CrosstermBackend};

#[tokio::main]
async fn main() -> Result<()> {
    let mut terminal = terminal::setup()?;
    let result = run_app(&mut terminal).await;
    terminal::restore(&mut terminal)?;
    result
}

async fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    let mut app = App::new()?;
    app.search().await;

    while !app.should_quit {
        terminal.draw(|frame| ui::draw(frame, &app))?;

        if event::poll(Duration::from_millis(100))? {
            let Event::Key(key) = event::read()? else {
                continue;
            };
            input::handle_key(&mut app, key).await;
        }
    }

    Ok(())
}
