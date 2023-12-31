use crate::app::{App, BookEditFocus, SearchFieldFocus};
use crate::database;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Spans,
    widgets::{Block, Borders, Clear, Paragraph, Row, Table},
    Frame,
};

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // Render header, body and footer of booky.
    render_main(app, frame);

    // Render activated popups
    render_add_popup(app, frame);
    render_help_popup(app, frame);
    render_search_popup(app, frame);
}

fn render_main<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
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

    // Show stats temp on the top
    let menu_block = Block::default()
        .title("Booky")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    let books_count = Spans::from(format!("Total: {}", app.items.len()));

    let stats_block = Paragraph::new(books_count).block(menu_block);
    frame.render_widget(stats_block, chunks[0]);

    if !app.search_active {
        app.items = database::get_books(app);
    }
    let rows: Vec<Row> = app
        .items
        .iter()
        .map(|i| {
            Row::new(vec![
                i.id.to_string(),
                i.title.to_string(),
                i.author.to_string(),
                i.genre.to_string(),
                i.rating.to_string(),
                i.status.to_string(),
                i.start_date.unwrap().to_string(),
                i.end_date.unwrap().to_string(),
                i.end_date
                    .unwrap()
                    .signed_duration_since(i.start_date.unwrap())
                    .num_days()
                    .to_string(),
            ])
        })
        .collect();
    let headers = Row::new(vec![
        "Id",
        "Title",
        "Author",
        "Genre",
        "Rating",
        "Status",
        "StartDate",
        "EndDate",
        "Finished (days)",
    ]);

    let table = Table::new(rows)
        .header(headers.style(Style::default().fg(Color::Yellow)))
        .block(Block::default().borders(Borders::ALL))
        .column_spacing(3)
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">>")
        .widths(&[
            Constraint::Length(2),
            Constraint::Length(20),
            Constraint::Length(20),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(15),
        ]);

    frame.render_stateful_widget(table, chunks[1], &mut app.state);

    let footer = Block::default().title("Log").borders(Borders::ALL);
    let temp_info = Paragraph::new("Press ? to access the help menu").block(footer);
    frame.render_widget(temp_info, chunks[2]);
}

fn render_add_popup<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    if app.add_popup {
        let block = Block::default().title("Add New Book").borders(Borders::ALL);
        let area = centered_rect(45, 60, frame.size());
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
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(2),
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
                .split(layout[8]);

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

            let tab_info = Paragraph::new("[tab] / [shift + tab] to cycle");
            frame.render_widget(tab_info, layout[7]);

            let b1 = Block::default().title("Title").borders(Borders::ALL);
            let b2 = Block::default().title("Author").borders(Borders::ALL);
            let b3 = Block::default().title("Genre").borders(Borders::ALL);
            let b4 = Block::default().title("Rating").borders(Borders::ALL);
            let b5 = Block::default().title("Status").borders(Borders::ALL);
            let b6 = Block::default()
                .title("StartDate (year/month/day)")
                .borders(Borders::ALL);
            let b7 = Block::default()
                .title("EndDate (year/month/day)")
                .borders(Borders::ALL);

            task.title.set_cursor_line_style(Style::default());
            task.author.set_cursor_line_style(Style::default());
            task.genre.set_cursor_line_style(Style::default());
            task.rating.set_cursor_line_style(Style::default());
            task.status.set_cursor_line_style(Style::default());
            task.start_date.set_cursor_line_style(Style::default());
            task.end_date.set_cursor_line_style(Style::default());

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

            task.genre.set_block(b3);
            if let BookEditFocus::Genre = task.focus {
                task.genre
                    .set_style(Style::default().add_modifier(Modifier::BOLD));
                task.genre
                    .set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
            } else {
                task.genre.set_style(Style::default());
                task.genre.set_cursor_style(Style::default());
            }
            frame.render_widget(task.genre.widget(), layout[2]);

            task.rating.set_block(b4);
            if let BookEditFocus::Rating = task.focus {
                task.rating
                    .set_style(Style::default().add_modifier(Modifier::BOLD));
                task.rating
                    .set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
            } else {
                task.rating.set_style(Style::default());
                task.rating.set_cursor_style(Style::default());
            }
            frame.render_widget(task.rating.widget(), layout[3]);

            task.status.set_block(b5);
            if let BookEditFocus::Status = task.focus {
                task.status
                    .set_style(Style::default().add_modifier(Modifier::BOLD));
                task.status
                    .set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
            } else {
                task.status.set_style(Style::default());
                task.status.set_cursor_style(Style::default());
            }
            frame.render_widget(task.status.widget(), layout[4]);

            task.start_date.set_block(b6);
            if let BookEditFocus::StartDate = task.focus {
                task.start_date
                    .set_style(Style::default().add_modifier(Modifier::BOLD));
                task.start_date
                    .set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
            } else {
                task.start_date.set_style(Style::default());
                task.start_date.set_cursor_style(Style::default());
            }
            frame.render_widget(task.start_date.widget(), layout[5]);

            task.end_date.set_block(b7);
            if let BookEditFocus::EndDate = task.focus {
                task.end_date
                    .set_style(Style::default().add_modifier(Modifier::BOLD));
                task.end_date
                    .set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
            } else {
                task.end_date.set_style(Style::default());
                task.end_date.set_cursor_style(Style::default());
            }
            frame.render_widget(task.end_date.widget(), layout[6]);
        }
    }
}

fn render_search_popup<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    if app.search_popup {
        let block = Block::default()
            .title("Search/Filter")
            .borders(Borders::ALL);
        let area = centered_rect(40, 55, frame.size());
        let block_inner = block.inner(area);
        frame.render_widget(Clear, area);
        frame.render_widget(Paragraph::new("").block(block), area);
        if let Some(task) = &mut app.search_field_state {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
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
                .split(layout[7]);

            let (create_style, cancel_style, create_txt, cancel_txt) = match task.focus {
                SearchFieldFocus::ConfirmBtn => (
                    Style::default().add_modifier(Modifier::BOLD),
                    Style::default(),
                    "[Confirm]",
                    " Cancel ",
                ),
                SearchFieldFocus::CancelBtn => (
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

            task.title.set_cursor_line_style(Style::default());

            task.title.set_block(b1);

            if let SearchFieldFocus::Title = task.focus {
                task.title
                    .set_style(Style::default().add_modifier(Modifier::BOLD));
                task.title
                    .set_cursor_style(Style::default().add_modifier(Modifier::REVERSED))
            } else {
                task.title.set_style(Style::default());
                task.title.set_cursor_style(Style::default());
            }
            frame.render_widget(task.title.widget(), layout[0]);

            let b2 = Block::default().title("Author").borders(Borders::ALL);

            task.author.set_cursor_line_style(Style::default());

            task.author.set_block(b2);

            if let SearchFieldFocus::Author = task.focus {
                task.author
                    .set_style(Style::default().add_modifier(Modifier::BOLD));
                task.author
                    .set_cursor_style(Style::default().add_modifier(Modifier::REVERSED))
            } else {
                task.author.set_style(Style::default());
                task.author.set_cursor_style(Style::default());
            }
            frame.render_widget(task.author.widget(), layout[1]);

            let b3 = Block::default().title("Genre").borders(Borders::ALL);

            task.genre.set_cursor_line_style(Style::default());

            task.genre.set_block(b3);

            if let SearchFieldFocus::Genre = task.focus {
                task.genre
                    .set_style(Style::default().add_modifier(Modifier::BOLD));
                task.genre
                    .set_cursor_style(Style::default().add_modifier(Modifier::REVERSED))
            } else {
                task.genre.set_style(Style::default());
                task.genre.set_cursor_style(Style::default());
            }
            frame.render_widget(task.genre.widget(), layout[2]);

            let b4 = Block::default().title("Rating").borders(Borders::ALL);

            task.rating.set_cursor_line_style(Style::default());

            task.rating.set_block(b4);

            if let SearchFieldFocus::Rating = task.focus {
                task.rating
                    .set_style(Style::default().add_modifier(Modifier::BOLD));
                task.rating
                    .set_cursor_style(Style::default().add_modifier(Modifier::REVERSED))
            } else {
                task.rating.set_style(Style::default());
                task.rating.set_cursor_style(Style::default());
            }
            frame.render_widget(task.rating.widget(), layout[3]);

            let b5 = Block::default().title("Status").borders(Borders::ALL);

            task.status.set_cursor_line_style(Style::default());

            task.status.set_block(b5);

            if let SearchFieldFocus::Status = task.focus {
                task.status
                    .set_style(Style::default().add_modifier(Modifier::BOLD));
                task.status
                    .set_cursor_style(Style::default().add_modifier(Modifier::REVERSED))
            } else {
                task.status.set_style(Style::default());
                task.status.set_cursor_style(Style::default());
            }
            frame.render_widget(task.status.widget(), layout[4]);

            let b6 = Block::default()
                .title("StartDate (year/month/day)")
                .borders(Borders::ALL);

            task.start_date.set_cursor_line_style(Style::default());

            task.start_date.set_block(b6);

            if let SearchFieldFocus::StartDate = task.focus {
                task.start_date
                    .set_style(Style::default().add_modifier(Modifier::BOLD));
                task.start_date
                    .set_cursor_style(Style::default().add_modifier(Modifier::REVERSED))
            } else {
                task.start_date.set_style(Style::default());
                task.start_date.set_cursor_style(Style::default());
            }
            frame.render_widget(task.start_date.widget(), layout[5]);

            let b7 = Block::default()
                .title("EndDate (year/month/day)")
                .borders(Borders::ALL);

            task.end_date.set_cursor_line_style(Style::default());

            task.end_date.set_block(b7);

            if let SearchFieldFocus::EndDate = task.focus {
                task.end_date
                    .set_style(Style::default().add_modifier(Modifier::BOLD));
                task.end_date
                    .set_cursor_style(Style::default().add_modifier(Modifier::REVERSED))
            } else {
                task.end_date.set_style(Style::default());
                task.end_date.set_cursor_style(Style::default());
            }
            frame.render_widget(task.end_date.widget(), layout[6]);
        }
    }
}

fn render_help_popup<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    if app.help_popup {
        let block = Block::default().title("Help").borders(Borders::ALL);
        let area = centered_rect(60, 40, frame.size());
        let block_inner = block.inner(area);
        frame.render_widget(Clear, area);
        frame.render_widget(Paragraph::new("").block(block), area);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                ]
                .as_ref(),
            )
            .split(block_inner);

        let p1 = Paragraph::new("i -> Add a new book");
        let p2 = Paragraph::new("d -> Delete current highlighted book (no confirmation)");
        let p3 = Paragraph::new("jk/updown arrows -> to select a book");
        let p4 = Paragraph::new("u -> Update current selected book");
        let p5 = Paragraph::new("/ -> Search for a book");
        let p6 = Paragraph::new("r -> Clear search");
        let p7 = Paragraph::new("q -> Quit");

        frame.render_widget(p1, layout[0]);
        frame.render_widget(p2, layout[1]);
        frame.render_widget(p3, layout[2]);
        frame.render_widget(p4, layout[3]);
        frame.render_widget(p5, layout[4]);
        frame.render_widget(p6, layout[5]);
        frame.render_widget(p7, layout[6]);
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
