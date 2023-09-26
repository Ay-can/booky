use booky::app::{App, AppResult};

use booky::database;
use booky::event::{Event, EventHandler};
use booky::handler::handle_key_events;
use booky::tui::Tui;
use diesel::{connection, Connection, SqliteConnection};
use diesel_migrations::{FileBasedMigrations, MigrationHarness};
use dirs_2::document_dir;
use std::error::Error;
use std::{fs, io};
use tui::backend::CrosstermBackend;
use tui::Terminal;

// Checks if db exists in documents folder, if it
// doesn't it will create one and return the string
fn dir_init() -> Result<String, Box<dyn Error + 'static>> {
    let booky_dir = document_dir()
        .expect("Failed to create booky directory in /Documents")
        .join("booky");

    if !booky_dir.exists() {
        fs::create_dir(booky_dir).expect("Failed to create booky directory");
    }

    let document_path = document_dir()
        .expect("Failed to find /Documents")
        .join("booky")
        .join("books.db");

    Ok(document_path.display().to_string())
}

fn main() -> AppResult<()> {
    if let Ok(s) = dir_init() {
        let connection = &mut SqliteConnection::establish(&s).expect("Could not connect to db");
        let migrations =
            FileBasedMigrations::find_migrations_directory().expect("Could not find migrations");
        connection
            .run_pending_migrations(migrations)
            .expect("Error running migrations");
    }

    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
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
