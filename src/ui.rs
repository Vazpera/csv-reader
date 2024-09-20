use std::{num::ParseFloatError, vec};

use crossterm::style::Color;
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Style, Stylize},
    symbols::Marker,
    text::{Line, Span},
    widgets::{
        self, Axis, Block, Dataset, LegendPosition, Paragraph, Row, StatefulWidget, TableState,
    },
    Frame,
};

use crate::{app::App, theme};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame, theme: theme::Theme) {
    let styles: Vec<Style> = theme
        .extra_colors
        .clone()
        .into_iter()
        .map(|x| Style::new().fg(x))
        .collect();
    let [path_bar, main] =
        Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).areas(frame.area());
    let [content, controls] =
        Layout::horizontal([Constraint::Percentage(80), Constraint::Percentage(20)]).areas(main);

    let bottom_title = Line::from(vec![
        Span::raw("Header Row: "),
        match app.has_header_row {
            true => Span::styled("TRUE ", styles[2]),
            false => Span::styled("FALSE ", styles[0]),
        },
        Span::raw("Header Column: "),
        match app.has_label_col {
            true => Span::styled("TRUE", styles[2]),
            false => Span::styled("FALSE", styles[0]),
        },
    ]);

    if app.is_graph {
        let mut encountered_err: Option<ParseFloatError> = None;
        let rows = app.value_matrix.clone().len();
        let cols = app
            .value_matrix
            .clone()
            .iter()
            .map(|v| v.len())
            .max()
            .unwrap_or(0);

        let mut transpose = vec![vec![String::new(); rows]; cols];

        // Transpose the data
        for (i, col) in app.value_matrix.clone().iter().enumerate() {
            for (j, val) in col.clone().into_iter().enumerate() {
                transpose[j][i] = val.clone();
            }
        }

        // Create a new vector with the correct size
        let mut result = vec![
            vec![String::new(); if app.has_header_row { rows - 1 } else { rows }];
            if app.has_label_col { cols - 1 } else { cols }
        ];

        // Transpose the data
        for (i, col) in app
            .value_matrix
            .clone()
            .iter()
            .skip(if app.has_header_row { 1 } else { 0 })
            .enumerate()
        {
            for (j, val) in col
                .clone()
                .into_iter()
                .skip(if app.has_label_col { 1 } else { 0 })
                .enumerate()
            {
                result[j][i] = val.clone();
            }
        }
        let mut upper_y = 0.0;
        let mut lower_y = f64::INFINITY;
        let datas: Vec<Vec<(f64, f64)>> = result
            .clone()
            .into_iter()
            .map(|x| {
                let y = x.into_iter().map(|x| match x.parse::<f64>() {
                    Ok(j) => {
                        if j > upper_y {
                            upper_y = j;
                        };
                        if j < lower_y {
                            lower_y = j;
                        };
                        j
                    }
                    Err(j) => {
                        encountered_err = Some(j);
                        0.0
                    }
                });
                return y.collect::<Vec<f64>>();
            })
            .collect::<Vec<Vec<f64>>>()
            .into_iter()
            .map(|x| {
                x.into_iter()
                    .enumerate()
                    .map(|(i, x)| (i as f64, x))
                    .collect::<Vec<(f64, f64)>>()
            })
            .collect();

        if let Some(j) = encountered_err {
            frame.render_widget(
                Block::bordered()
                    .title(format!("Encountered Error: {}", j))
                    .title_alignment(Alignment::Center)
                    .title_bottom(bottom_title)
                    .bg(theme.background)
                    .border_style(Style::new().fg(theme.border)),
                if app.controls { content } else { frame.area() },
            );
        } else {
            let mut datasets: Vec<Dataset> = Vec::new();
            for (i, _) in datas.clone().into_iter().enumerate() {
                datasets.push(
                    Dataset::default()
                        .data(&datas[i])
                        .graph_type(widgets::GraphType::Line)
                        .name(if app.has_header_row {
                            app.value_matrix[0][i + if app.has_label_col { 1 } else { 0 }].clone()
                        } else {
                            " ".to_string()
                        })
                        .marker(Marker::Braille)
                        .style(styles[i % styles.len()]),
                );
            }
            let upper_x = (datas[0].len() - 1) as f64;
            let lower_x = 0.0;
            let chart = widgets::Chart::new(datasets)
                .legend_position(if app.has_header_row {
                    Some(LegendPosition::BottomLeft)
                } else {
                    None
                })
                .x_axis(
                    Axis::default()
                        .bounds([lower_x, upper_x])
                        .labels(if app.has_label_col {
                            transpose.clone()[0]
                                .clone()
                                .into_iter()
                                .skip(if app.has_header_row { 1 } else { 0 })
                                .map(|x| x.fg(theme.text))
                                .collect::<Vec<Span>>()
                        } else {
                            Vec::new()
                        })
                        .title(if app.has_label_col && app.has_header_row {
                            app.value_matrix[0][0].clone()
                        } else {
                            String::new()
                        })
                        .fg(theme.text),
                )
                .y_axis(
                    Axis::default()
                        .bounds([lower_y, upper_y])
                        .labels([
                            format!("{}", lower_y).fg(theme.text),
                            format!("{}", upper_y).fg(theme.text),
                        ])
                        .fg(theme.text),
                )
                .block(
                    Block::bordered()
                        .title_alignment(Alignment::Center)
                        .title_bottom(bottom_title)
                        .bg(theme.background)
                        .border_style(Style::new().fg(theme.border)),
                )
                .bg(theme.background);

            frame.render_widget(chart, if app.controls { content } else { frame.area() });
        }
    } else {
        if app.value_matrix.len() == 0 {
        } else {
            let constraints: Vec<Constraint> = app.value_matrix.clone()[0]
                .clone()
                .into_iter()
                .map(|_| Constraint::Fill(1))
                .collect();

            let rows = app
                .value_matrix
                .clone()
                .into_iter()
                .enumerate()
                .map(|(i, x)| {
                    let row = Row::new(x.into_iter().enumerate().map(|(j, x)| {
                        if (j, i) == app.current_location {
                            match app.editing {
                                true => app
                                    .current_value
                                    .clone()
                                    .bold()
                                    .bg(theme.header_background)
                                    .fg(theme.header_text),
                                false => {
                                    app.current_value.clone().bold().underlined().fg(theme.text)
                                }
                            }
                        } else if j == 0 && app.has_label_col {
                            x.bold().fg(theme.text)
                        } else {
                            x.fg(theme.text)
                        }
                    }));
                    if i % 2 == 0 {
                        row.bg(theme.highlight_background).fg(theme.highlight_text)
                    } else {
                        row
                    }
                });

            let mut boxes_state = TableState::default();
            boxes_state.scroll_down_by(app.current_location.1 as u16);
            let _ = widgets::Table::new(
                rows.clone()
                    .skip(if app.has_header_row { 1 } else { 0 })
                    .collect::<Vec<Row>>(),
                constraints,
            )
            .block(
                Block::bordered()
                .title_alignment(Alignment::Center)
                    .title_bottom(bottom_title)
                    .bg(theme.background)
                    .border_style(Style::new().fg(theme.border)),
            )
            .header(if app.has_header_row {
                rows.collect::<Vec<Row>>()[0]
                    .clone()
                    .fg(theme.header_text)
                    .bg(theme.header_background)
                    .italic()
            } else {
                Row::default()
            })
            .bg(theme.background)
            .render(
                if app.controls { content } else { frame.area() },
                frame.buffer_mut(),
                &mut boxes_state,
            );
        }
    }
    if app.controls {
        let _ = widgets::Table::new(
            match app.editing {
                false => vec![
                    Row::new(vec!["Enter", "Enter Editing"]).fg(theme.text),
                    Row::new(vec!["Arrows", "Move Selection"]).fg(theme.text),
                    Row::new(vec!["q/CTR+C", "Exit"]).fg(theme.text),
                    Row::new(vec!["CTR+Z", "Undo"]).fg(theme.text),
                    Row::new(vec!["h", "Toggle Header Row"]).fg(theme.text),
                    Row::new(vec!["j", "Toggle Label Col"]).fg(theme.text),
                    Row::new(vec!["y", "Add Row"]).fg(theme.text),
                    Row::new(vec!["n", "Remove Row"]).fg(theme.text),
                    Row::new(vec!["u", "Add Col"]).fg(theme.text),
                    Row::new(vec!["m", "Remove Col"]).fg(theme.text),
                    Row::new(vec!["k", "Toggle Graph"]).fg(theme.text),
                    Row::new(vec!["c", "Toggle Control Panel"]).fg(theme.text),
                ],
                true => vec![
                    Row::new(vec!["Enter", "Exit Editing"]).fg(theme.text),
                    Row::new(vec!["Arrows", "Move Cursor"]).fg(theme.text),
                    Row::new(vec!["CTR+C", "Exit"]).fg(theme.text),
                ],
            },
            [Constraint::Fill(1), Constraint::Fill(3)],
        )
        .block(
            Block::bordered()
                .title("Controls Panel")
                .title_alignment(Alignment::Center)
                .title_bottom(format!("Edits: {}", app.previous_matrices.len()))
                .bg(theme.background)
                .border_style(Style::new().fg(theme.border)),
        )
        .bg(theme.background)
        .render(controls, frame.buffer_mut(), &mut TableState::default());
    }
    let path = Paragraph::new(app.path.clone())
        .fg(theme.path_text)
        .bg(theme.path_background);
    frame.render_widget(path, path_bar);
}
