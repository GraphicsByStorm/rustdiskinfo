use ratatui::{
    backend::Backend,
    widgets::{Block, Borders, Paragraph},
    layout::{Constraint, Direction, Layout},
    Frame,
};

pub fn show_menu<B: Backend>(frame: &mut Frame<B>) {
    let block = Block::default()
        .title("Rust Disk Info")
        .borders(Borders::ALL);
    frame.render_widget(block, frame.size());
}
