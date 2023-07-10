use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(frame.size());

    let menu_block = Block::default()
        .title("Booky")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    frame.render_widget(menu_block, chunks[0]);

    let main_block = Block::default().title("Books").borders(Borders::ALL);
    frame.render_widget(main_block, chunks[1]);

    let footer = Block::default().title("Footer").borders(Borders::ALL);
    frame.render_widget(footer, chunks[2]);
}
