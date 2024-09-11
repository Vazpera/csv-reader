
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{ Style, Stylize},
    widgets::{self, Block,Row, TableState, StatefulWidget},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let [content, controls] =
        Layout::horizontal([Constraint::Percentage(80), Constraint::Percentage(20)])
            .areas(frame.area());
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
            Row::new(x.into_iter().enumerate().map(|(j, x)| {
                if (j, i) == app.current_location {
                    match app.editing {
                        true => app.current_value.clone().bold(),
                        false => app.current_value.clone().bold().on_red(),
                    }
                } else {
                    x.into()
                }
            }))
        });
    
    let mut boxes_state = TableState::default();
    boxes_state.scroll_down_by(app.current_location.1 as u16);
    let _ = widgets::Table::new(rows, constraints)
        .highlight_style(Style::new().on_red())
        
        .block(
            Block::bordered()
                .title(app.path.clone())
                .title_alignment(Alignment::Center),
        )
        .render(content, frame.buffer_mut(), &mut boxes_state);
    let _ = widgets::Table::new(
        match app.editing {
            false => vec![
                Row::new(vec!["Enter", "Enter Editing"]),
                Row::new(vec!["Arrows", "Move Selection"]),
                Row::new(vec!["Q/CTR+C", "Exit"]),
            ],
            true => vec![
                Row::new(vec!["Enter", "Exit Editing"]),
                Row::new(vec!["Arrows", "Move Cursor"]),
                Row::new(vec!["CTR+C", "Exit"]),
            ],
        },
        [Constraint::Fill(1), Constraint::Fill(1)],
    )
    .block(
        Block::bordered()
            .title("Controls")
            .title_alignment(Alignment::Center),
    )
    .render(controls, frame.buffer_mut(), &mut TableState::default());
}
