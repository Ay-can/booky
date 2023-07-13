use crate::app::{App, Book, BookEditFocus};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Cell, Clear, Paragraph, Row, Table},
    Frame,
};
use tui_textarea::TextArea;

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

    let book_list = match app.read_json() {
        Ok(t) => t,
        Err(_) => app.items.clone(),
    };
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
    let info_box = Paragraph::new("Add: 'a'").block(footer);
    frame.render_widget(info_box, chunks[2]);

    // Popup
    if app.show_popup {
        let block = Block::default().title("Add New Book").borders(Borders::ALL);
        let area = centered_rect(60, 30, frame.size());
        let block_inner = block.inner(area);
        frame.render_widget(Clear, area);
        frame.render_widget(Paragraph::new("").block(block), area);
        if let Some(task) = &mut app.book_edit_state {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(1),
                        Constraint::Length(2),
                    ]
                    .as_ref(),
                )
                .split(block_inner);

            let buttons = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(80),
                        Constraint::Min(10),
                        Constraint::Min(10),
                    ]
                    .as_ref(),
                )
                .split(layout[2]);

            let (create_style, cancel_style, create_txt, cancel_txt) = match task.focus {
                BookEditFocus::ConfirmBtn => (
                    Style::default().add_modifier(Modifier::BOLD),
                    Style::default(),
                    "[Confirm]",
                    " Cancel ",
                ),
                BookEditFocus::CancelBtn => (
                    Style::default(),
                    Style::default().add_modifier(Modifier::BOLD),
                    " Confirm ",
                    "[Cancel]",
                ),
                _ => (Style::default(), Style::default(), " Confirm ", " Cancel "),
            };

            let create_btn = Paragraph::new(create_txt).style(create_style);
            let cancel_btn = Paragraph::new(cancel_txt).style(cancel_style);
            frame.render_widget(create_btn, buttons[1]);
            frame.render_widget(cancel_btn, buttons[2]);

            let b1 = Block::default().title("Title").borders(Borders::ALL);
            let b2 = Block::default().title("Author").borders(Borders::ALL);
            let b3 = Block::default().title("Keys").borders(Borders::ALL);

            task.title.set_cursor_line_style(Style::default());
            task.author.set_cursor_line_style(Style::default());

            task.title.set_block(b1);

            if let BookEditFocus::Title = task.focus {
                task.title
                    .set_style(Style::default().add_modifier(Modifier::BOLD));
                task.title
                    .set_cursor_style(Style::default().add_modifier(Modifier::REVERSED))
            } else {
                task.title.set_style(Style::default());
                task.title.set_cursor_style(Style::default());
            }
            frame.render_widget(task.title.widget(), layout[0]);

            task.author.set_block(b2);
            if let BookEditFocus::Author = task.focus {
                task.author
                    .set_style(Style::default().add_modifier(Modifier::BOLD));
                task.author
                    .set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
            } else {
                task.author.set_style(Style::default());
                task.author.set_cursor_style(Style::default());
            }
            frame.render_widget(task.author.widget(), layout[1]);
        }
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
