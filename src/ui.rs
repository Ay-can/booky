use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Row, Table},
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

    let table = Table::new(vec![
        Row::new(vec![
            "Can't Hurt Me",
            "David Goggins",
            "Self-help",
            "10",
            "14-05-2020",
            "17-05-2020",
        ]),
        Row::new(vec![
            "Deepwork",
            "Cal Newport",
            "Self-help",
            "9",
            "test",
            "test",
        ]),
    ])
    .style(Style::default().fg(Color::White))
    .header(
        Row::new(vec![
            "Title", "Author", "Genre", "Rating", "Start", "Finish",
        ])
        .style(Style::default().fg(Color::Yellow)),
    )
    .block(Block::default().title("Books").borders(Borders::ALL))
    .widths(&[
        Constraint::Length(20),
        Constraint::Length(15),
        Constraint::Length(10),
        Constraint::Length(10),
        Constraint::Length(10),
        Constraint::Length(10),
    ])
    .column_spacing(10)
    .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .highlight_symbol(">>");
    frame.render_stateful_widget(table, chunks[1], &mut app.state);

    let footer = Block::default().title("Footer").borders(Borders::ALL);
    frame.render_widget(footer, chunks[2]);
}
