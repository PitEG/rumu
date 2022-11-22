use std::{
    thread,
    io, 
    time::Duration};
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, List, ListState, ListItem, Gauge},
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
use crate::app::command::{Command,Response};
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

        
        let mut songlist : SongList = SongList::new(self.songs.search_all());
        songlist.order_items(SongOrder::Album);

        let mut songlist_state = ListState::default();
        songlist_state.select(Some(songlist.get_selection() as usize));

        let mut songqueue : SongQueue = SongQueue::new();
        let mut songqueue_state = ListState::default();
        songqueue_state.select(None);

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
                let queue = queue_to_tui_list(&songqueue);

                f.render_stateful_widget(queue, right_top_chunk, &mut songqueue_state);
                f.render_widget(block.clone(), right_bottom_chunk);
                f.render_stateful_widget(list, center_chunk, &mut songlist_state);
                f.render_widget(block.clone(), center_top_chunk);
                f.render_widget(nav_to_tui_list(&self.nav), left_chunk);
                f.render_widget(song_detail(&self.player), bottom_chunk);
            })?;

            // read input
            let curr_panel : &mut dyn command::Command = match panel {
                SelectedPanel::SongList => &mut songlist,
                SelectedPanel::Queue => &mut songqueue,
                _ => &mut songlist,
            };

            let mut response : Option<Response> = None;
            match crossterm::event::poll(Duration::new(0,10000)) {
                Ok(true) => {
                    match crossterm::event::read()? {
                        Event::Key(event) => {
                            // commands for pannel
                            let command : command::Event = match event.code {
                                KeyCode::Esc => {break;}, // breaks out of loop
                                KeyCode::Up => command::Event::Up,
                                KeyCode::Down => command::Event::Down,
                                KeyCode::Enter => command::Event::Accept,
                                KeyCode::Left => command::Event::Left,
                                KeyCode::Right => command::Event::Right,
                                _ => command::Event::N, // else do nothing
                            };

                            // signal panel with command
                            response = curr_panel.command(&command);

                            // commands for app
                            match event.code {
                                KeyCode::Char('p') => {
                                    self.player.stop().ok();
                                    let song = songqueue.queue.get(0);
                                    match song {
                                        Some(x) => { 
                                            self.player.play(&x.path[..]).ok(); 
                                            songqueue.set_currently_playing(0);
                                        },
                                        _ => {}
                                    }
                                },
                                KeyCode::Char('q') => {
                                    panel = SelectedPanel::Queue;
                                },
                                KeyCode::Char('w') => {
                                    panel = SelectedPanel::SongList;
                                },
                                _ => {}
                            }

                            // println!("{:?}", event);
                        },
                        // Event::Mouse(event) => println!("{:?}", event),
                        // Event::Resize(width, height) => println!("New size {}x{}", width, height),
                        // Event::Paste(data) => println!("{:?}", data),
                        _ => {}, // else do nothing else
                    }
                },
                _ => {},
            }

            // process command
            match response {
                Some(r) => {
                    match r {
                        Response::PlaySong(s) => {
                            let _ = self.player.stop();
                            let _ = self.player.play(&s.path[..]);
                        }
                        Response::QueueSong(s) => {
                            songqueue.push(s);
                        }
                        _ => {},
                    }
                }
                _ => {},
            }
            songlist_state.select(Some(songlist.get_selection() as usize));
            // songqueue_state.select(Some(songqueue.get_selection() as usize));
            match songqueue.get_selection() {
                Some(v) => songqueue_state.select(Some(v as usize)),
                None => songqueue_state.select(None)
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

fn queue_to_tui_list(q : &songqueue::SongQueue) -> List {
    let mut item_list : Vec<ListItem> = q.queue.iter().map(|x| ListItem::new(x.to_string())).collect();
    match q.get_currently_playing() {
        Some(v) => { 
            let playing_item = item_list[v as usize].clone().style(Style::default().fg(Color::Green)); 
            item_list[v as usize] = playing_item;
        }
        None => {},
    }
    let list = List::new(item_list)
        .block(Block::default().title("list of stuf").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_symbol(">>");
    return list;
}

fn song_detail(player : &player::Player) -> Gauge {
    let time_left = player.get_time_left();
    let duration = player.get_song_duration();
    let mut fraction_played = (1.0 - time_left / duration).clamp(0.0,1.0);
    if fraction_played.is_nan() {
        fraction_played = 0.0;
    }
    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Progress"))
        .gauge_style(Style::default().fg(Color::White).bg(Color::Black).add_modifier(Modifier::ITALIC))
        .ratio(fraction_played);
    // let gauge = Paragraph::new(Text::from(fraction_played.to_string()));
    return gauge;
}

pub fn create(songdb: SongDB) -> App {
    let player = player::new();
    let nav: Navigator = Navigator::new(); 
    let app = App {
        songs: songdb,
        nav,
        player
    };
    return app;
}
