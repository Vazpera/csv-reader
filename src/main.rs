use std::{
    fs,
    io,
};

use crate::{
    app::{App, AppResult},
    event::{Event, EventHandler},
    handler::handle_key_events,
    theme::Theme,
    tui::Tui,
};
use ratatui::{backend::CrosstermBackend, Terminal};

pub mod app;
pub mod event;
pub mod handler;
pub mod theme;
pub mod tui;
pub mod ui;
use std::env;

#[tokio::main]
async fn main() -> AppResult<()> {
    let args: Vec<String> = env::args().collect();
    // Create an application.
    let mut app = App::new(args[1].clone());
    let source_directory = env::current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().to_string_lossy().to_string();
    let theme_string = fs::read_to_string(source_directory+"/theme.json");
    let theme: Theme = match serde_json::from_str(&theme_string.unwrap().as_str()) {
        Ok(them) => them,
        Err(_) => serde_json::from_str(
            "
        {
    \"border\": \"Gray\",
    \"background\": \"Reset\",
    \"text\": \"Gray\",
    \"header_background\": \"Red\",
    \"header_text\": \"White\",
    \"highlight_background\": \"DarkGray\",
    \"highlight_text\": \"White\",
    \"path_background\": \"Reset\",
    \"path_text\": \"Gray\",
    \"extra_colors\": [\"red\", \"yellow\", \"green\", \"blue\", \"magenta\"]
}",
        )
        .unwrap(),
    };

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app, theme.clone())?;
        // Handle events.
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
