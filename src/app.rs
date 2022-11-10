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
    player: player::Player
}

impl App {
    pub fn start(&mut self) -> Result<(), io::Error> {
        // sample tui code
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let mut state = ListState::default();
        state.select(None);
        let mut result_list = self.songs.search_all();

        loop {
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

                let list = song_list_to_tui_list(&result_list);

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
                                None => 1,
                            };
                            state.select(Some(std::cmp::min(88,curr.overflowing_sub(1).0)));
                        }
                        KeyCode::Down => {
                            let curr = match state.selected() {
                                Some(v) => v,
                                None => 0,
                            };
                            state.select(Some((curr + 1) % 89));
                        }
                        KeyCode::Enter => {
                            self.player.stop().ok();
                            match state.selected() {
                                Some(x) => {
                                    self.player.play(&result_list[x].path[..]).ok();
                                }
                                None => {},
                            }
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
}

fn song_list_to_tui_list(song_list : &Vec<Song>) -> List {
    // let mut song_list = self.songs.search_all();
    // song_list.sort_by(|a,b| a.album.cmp(&b.album));
    let item_list : Vec<ListItem> = song_list.iter().map(|x| ListItem::new(x.to_string())).collect();
    let mut list = List::new(item_list)
        .block(Block::default().title("list of stuf").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_symbol(">>");
    return list;
}

pub fn create(songdb: SongDB) -> App {
    let player = player::new();
    let app = App {
        songs: songdb,
        player
    };
    return app;
}
