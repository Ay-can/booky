use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Cell, Clear, Paragraph, Row, Table},
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

    let book_list = app.read_json().expect("Failed to read books");
    let rows: Vec<Row> = book_list
        .iter()
        .map(|i| {
            Row::new(vec![
                i.id.to_string(),
                i.title.to_string(),
                i.author.to_string(),
                i.genre.to_string(),
                i.rating.to_string(),
            ])
        })
        .collect();
    let headers = Row::new(vec!["Id", "Title", "Author", "Genre", "Rating"]);

    let table = Table::new(rows)
        .header(headers.style(Style::default().fg(Color::Yellow)))
        .block(Block::default().borders(Borders::ALL))
        .column_spacing(5)
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">>")
        .widths(&[
            Constraint::Length(10),
            Constraint::Length(20),
            Constraint::Length(20),
            Constraint::Length(20),
            Constraint::Length(20),
        ]);

    frame.render_stateful_widget(table, chunks[1], &mut app.state);

    let footer = Block::default().title("Footer").borders(Borders::ALL);
    let info_box = Paragraph::new("Add: 'n'").block(footer);
    frame.render_widget(info_box, chunks[2]);

    // Popup
    if app.show_popup {
        let block = Block::default()
            .title("Create New Book")
            .borders(Borders::ALL);
        let area = centered_rect(60, 20, frame.size());
        frame.render_widget(Clear, area);
        frame.render_widget(block, area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
