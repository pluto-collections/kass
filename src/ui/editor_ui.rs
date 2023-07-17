use std::cmp::Ordering;

use tui::{
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{List, ListItem},
};

use crate::{app::App, enums::LineNumber};

pub fn editor_ui(app: &App) -> Vec<(List, Rect)> {
    let tab = &app.tabs[app.active_index];

    let mut list: Vec<(List, Rect)> = vec![];

    for (_i, buffer) in tab.buflist.iter().enumerate() {
        let line_numbers: Vec<ListItem> = match app.settings.line_number {
            LineNumber::Absolute => buffer
                .get_rows()
                .iter()
                .enumerate()
                .map(|(i, _m)| {
                    let number = vec![Spans::from(Span::styled(
                        format!("{}", i + 1 + buffer.offset.1 as usize),
                        Style::default().fg(Color::DarkGray),
                    ))];
                    ListItem::new(number)
                })
                .collect(),

            _ => buffer
                .get_rows()
                .iter()
                .enumerate()
                .map(|(i, _m)| {
                    // Displays the relative line number
                    let cursor_at = buffer.get_cursor().1 + buffer.offset.1;
                    let line_order = cursor_at.cmp(&i);

                    let relative_ln = match line_order {
                        Ordering::Equal => i + 1,
                        Ordering::Greater => cursor_at - i,
                        Ordering::Less => i - cursor_at,
                    };

                    let padding = app.settings.line_number_padding + 4;

                    let number = vec![Spans::from(Span::styled(
                        if line_order == Ordering::Equal {
                            format!("{:<padding$}", relative_ln, padding = padding)
                        } else {
                            format!("{:padding$}", relative_ln)
                        },
                        Style::default().fg(Color::DarkGray),
                    ))];

                    ListItem::new(number)
                })
                .collect(),
        };

        let rows: Vec<_> = buffer
            .get_rows()
            .iter()
            .map(|cells| {
                let cells: Vec<Span> = cells
                    .iter()
                    .map(|cell| {
                        Span::styled(
                            &cell.symbol,
                            Style::default()
                                .fg(cell.fg)
                                .bg(cell.bg)
                                .add_modifier(cell.modifier),
                        )
                    })
                    .collect();

                ListItem::new(Spans::from(cells))
            })
            .collect();

        let rows = List::new(rows);

        let (width, height) = buffer.size;
        let (x, y) = buffer.pos;

        let buffer_rect = Rect::new(
            x as u16,
            match app.settings.show_tabs {
                crate::enums::ShowTab::Never => 0,
                crate::enums::ShowTab::Always => 1,
                crate::enums::ShowTab::Multiple => {
                    if app.tabs.len() > 1 {
                        1
                    } else {
                        0
                    }
                }
            } + y as u16,
            width as u16,
            height as u16,
        );

        list.push((rows, rect))
    }

    list
}
