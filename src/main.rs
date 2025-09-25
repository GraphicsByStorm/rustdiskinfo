mod tools;
mod ui;

use crossterm::{execute, terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::{backend::{CrosstermBackend}, Terminal};

fn main() -> Result<(), std::io::Error> {
    enable_raw_mode()?;
    execute!(std::io::stdout(), EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        ui::show_menu(f);
    })?;

    disable_raw_mode()?;
    execute!(std::io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}