use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::songdb::SongDB;

pub struct Player {
    songs: SongDB,
}

impl Player {
    pub fn start(&self) -> Result<(), io::Error> {
        // sample tui code
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title("Block")
                .borders(Borders::ALL);
            f.render_widget(block, size);
        })?;

        thread::sleep(Duration::from_millis(5000));

        // restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        Ok(())
    }
}

pub fn create(songdb: SongDB) -> Player {
    let player = Player {songs: songdb};
    return player;
}
