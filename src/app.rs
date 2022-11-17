use std::{io, thread, time::Duration, collections::VecDeque};
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
use crate::app::navigator::{Navigator};
use crate::app::songlist::{SongList, SongOrder};
use crate::app::command::Command;
use crate::app::songqueue::SongQueue;

mod navigator;
mod command;
mod songlist;
mod songqueue;

enum SelectedPanel {
    SongList,
    Nav,
    Queue,
    Search,
}

pub struct App {
    songs: SongDB,
    queue: VecDeque<Song>,
    nav: Navigator,
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
        
        let mut songlist : SongList = SongList::new(self.songs.search_all());
        songlist.order_items(SongOrder::Album);
        state.select(Some(songlist.get_selection() as usize));

        let mut songqueue : SongQueue = SongQueue::new();

        let mut panel = SelectedPanel::SongList;

        loop {
            // draw 
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

                let list = song_list_to_tui_list(&songlist.items);
                let queue = queue_to_tui_list(&self.queue);

                f.render_widget(queue, right_top_chunk);
                f.render_widget(block.clone(), right_bottom_chunk);
                f.render_stateful_widget(list, center_chunk, &mut state);
                f.render_widget(block.clone(), center_top_chunk);
                f.render_widget(nav_to_tui_list(&self.nav), left_chunk);
                f.render_widget(block, bottom_chunk);
            })?;

            // read input
            let curr_panel : &mut dyn command::Command = match panel {
                SelectedPanel::SongList => &mut songlist,
                _ => &mut songqueue,
            };
            match crossterm::event::read()? {
                Event::Key(event) => {
                    match event.code {
                        KeyCode::Esc => {break;},
                        KeyCode::Up => {
                            let command : command::Event = command::Event::Up;
                            curr_panel.command(&command);
                            state.select(Some(songlist.get_selection() as usize));
                        }
                        KeyCode::Down => {
                            let command : command::Event = command::Event::Down;
                            curr_panel.command(&command);
                            state.select(Some(songlist.get_selection() as usize));
                        }
                        KeyCode::Enter => {
                            match state.selected() {
                                Some(x) => {
                                    self.queue.push_back(songlist.get_items()[x].clone());
                                }
                                None => {},
                            }
                        }
                        KeyCode::Char('p') => {
                            self.player.stop().ok();
                            match self.queue.get(0) {
                                Some(x) => { 
                                    self.player.play(&x.path[..]).ok(); 
                                },
                                _ => {}
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

            // process commands

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
    let list = List::new(item_list)
        .block(Block::default().title("list of stuf").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_symbol(">>");
    return list;
}

fn queue_to_tui_list(queue: &VecDeque<Song>) -> List {
    let item_list : Vec<ListItem> = queue.iter().map(|x| ListItem::new(x.to_string())).collect();
    let list = List::new(item_list)
        .block(Block::default().title("list of stuf").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_symbol(">>");
    return list;
}

fn nav_to_tui_list(nav: &navigator::Navigator) -> List {
    let mut item_list : Vec<ListItem> = Vec::new();
    item_list.push(ListItem::new("something"));
    let list = List::new(item_list)
        .block(Block::default().title("list of stuf").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_symbol(">>");
    return list;
}

pub fn create(songdb: SongDB) -> App {
    let player = player::new();
    let queue : VecDeque<Song> = VecDeque::new();
    let nav: Navigator = Navigator::new(); 
    let app = App {
        songs: songdb,
        nav,
        queue,
        player
    };
    return app;
}
