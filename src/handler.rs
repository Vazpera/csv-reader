use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.editing {
        false => {
            match key_event.code {
                // Exit application on `ESC` or `q`
                KeyCode::Esc | KeyCode::Char('q') => {
                    app.quit();
                }
                // Exit application on `Ctrl-C`

                // Counter handlers
                KeyCode::Right => app.move_right(),
                KeyCode::Left => app.move_left(),
                KeyCode::Up => app.move_up(),
                KeyCode::Down => app.move_down(),
                KeyCode::Enter => app.enter_editing(),
                KeyCode::Char('y') => app.add_row(),
                KeyCode::Char('n') => app.remove_row(),
                KeyCode::Char('h') => app.toggle_header_row(),
                KeyCode::Char('u') => app.add_col(),
                KeyCode::Char('m') => app.remove_col(),
                KeyCode::Char('j') => app.toggle_label_col(),
                KeyCode::Char('k') => app.toggle_graph_mode(),
                // Other handlers you could add here.
                KeyCode::Char('c') | KeyCode::Char('C') => {
                    if key_event.modifiers == KeyModifiers::CONTROL {
                        app.quit();
                    } else {
                        app.toggle_controls();
                    }
                }
                KeyCode::Char('z') | KeyCode::Char('Z') => {
                    if key_event.modifiers == KeyModifiers::CONTROL {
                        app.undo();
                    }
                }
                _ => {}
            }
        }
        true => {
            match key_event.code {
                KeyCode::Char(j) => {
                    if key_event.modifiers == KeyModifiers::CONTROL && j == 'c' {
                        app.quit();
                    }
                    app.edit(j);
                }
                KeyCode::Backspace => app.backspace(),
                KeyCode::Enter => app.exit_editing(),

                // Other handlers you could add here.
                _ => {}
            }
        }
    }

    Ok(())
}
