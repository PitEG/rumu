use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders, List, ListState, ListItem},
    layout::{Layout, Constraint, Direction},
    style::{Style, Color, Modifier}, 
    Terminal
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::songdb::SongDB;

pub struct App {
    songs: SongDB,
}

impl App {
    pub fn start(&self) -> Result<(), io::Error> {
        // sample tui code
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let mut state = ListState::default();
        state.select(Some(1));

        loop {
            let items = ["something", "else", "hello","wowie"];

            // draw loop
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10)
                        ].as_ref()
                        ).split(f.size());
                let block = Block::default()
                    .title("Block")
                    .borders(Borders::ALL);
                let list = List::new(items.map(|x| ListItem::new(x)))
                    .block(Block::default().title("list of stuf").borders(Borders::ALL))
                    .style(Style::default().fg(Color::White))
                    .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                    .highlight_symbol(">>");
                f.render_widget(block.clone(), chunks[0]);
                f.render_widget(block, chunks[2]);
                f.render_stateful_widget(list, chunks[1], &mut state);
            })?;

            // read input
            match crossterm::event::read()? {
                Event::Key(event) => {
                    match event.code {
                        KeyCode::Esc => {break;},
                        KeyCode::Up => {
                            let curr = match state.selected() {
                                Some(v) => v,
                                None => 0,
                            };
                            state = ListState::default();
                            state.select(Some(std::cmp::min(items.len()-1,curr.overflowing_sub(1).0)));
                        }
                        _ => {}, // else do nothing
                    }
                    // println!("{:?}", event);
                },
                // Event::Mouse(event) => println!("{:?}", event),
                // Event::Resize(width, height) => println!("New size {}x{}", width, height),
                // Event::Paste(data) => println!("{:?}", data),
                _ => {}, // else do nothing else
            }
            thread::sleep(Duration::from_millis(16));
        }

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

pub fn create(songdb: SongDB) -> App {
    let app = App {songs: songdb};
    return app;
}