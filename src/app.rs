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
use crate::player;
use crate::song::Song;

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

        let mut player = player::new();
        let mut state = ListState::default();
        state.select(Some(1));

        loop {
            let list = self.result_list();
            // draw loop
            terminal.draw(|f| {

                let main_chunk = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints([
                        Constraint::Percentage(85),
                        Constraint::Percentage(15)
                        ].as_ref())
                    .split(f.size());
                let top_chunk = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(0)
                    .constraints([
                                 Constraint::Percentage(20),
                                 Constraint::Percentage(60),
                                 Constraint::Percentage(20),
                        ].as_ref())
                    .split(main_chunk[0]);
                let right_chunk = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints([
                                 Constraint::Percentage(60),
                                 Constraint::Percentage(40),
                    ].as_ref())
                    .split(top_chunk[2]);
                let middle_chunk = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints([
                                 Constraint::Min(3),
                                 Constraint::Percentage(100)
                    ].as_ref())
                    .split(top_chunk[1]);
                let center_top_chunk = middle_chunk[0];
                let center_chunk = middle_chunk[1];
                let left_chunk = top_chunk[0];
                let right_top_chunk = right_chunk[0];
                let right_bottom_chunk = right_chunk[1];
                let bottom_chunk = main_chunk[1];
                let block = Block::default()
                    .title("Block")
                    .borders(Borders::ALL);

                f.render_widget(block.clone(), right_top_chunk);
                f.render_widget(block.clone(), right_bottom_chunk);
                f.render_stateful_widget(list, center_chunk, &mut state);
                f.render_widget(block.clone(), center_top_chunk);
                f.render_widget(block.clone(), left_chunk);
                f.render_widget(block, bottom_chunk);
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
                            state.select(Some(std::cmp::min(88,curr.overflowing_sub(1).0)));
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
            thread::sleep(Duration::from_millis(20));
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

    fn result_list(&self) -> List {
        let song_list = self.songs.search_all();
        let item_list : Vec<ListItem> = song_list.iter().map(|x| ListItem::new(x.to_string())).collect();
        let list = List::new(item_list)
            .block(Block::default().title("list of stuf").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_symbol(">>");
        return list;
    }
}

pub fn create(songdb: SongDB) -> App {
    let app = App {songs: songdb};
    return app;
}
