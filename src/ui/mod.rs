use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn show_menu(frame: &mut Frame<impl Backend>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(frame.size());

    let title = Block::default()
        .borders(Borders::ALL)
        .title("Drive List");

    let drive_list = Paragraph::new("Drive 1:... \nDrive 2:...")
        .block(Block::default().borders(Borders::ALL).title("Drives"));

    frame.render_widget(title, chunks[0]);
    frame.render_widget(drive_list, chunks[1]);
}