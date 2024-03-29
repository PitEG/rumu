use std::{
    thread,
    io, 
    time::Duration};
use tui::{
    backend::CrosstermBackend,
    widgets::{Paragraph, Block, Borders, List, ListState, ListItem, Gauge, LineGauge},
    layout::{Layout, Constraint, Direction, Rect},
    style::{Style, Color, Modifier}, 
    text::Text,
    terminal::Frame,
    Terminal
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::songdb;
use crate::songdb::SongDB;
use crate::player;
use crate::song::Song;
use crate::app::navigator::{Navigator};
use crate::app::songlist::{SongList, SongOrder};
use crate::app::command::{Command,Response};
use crate::app::songqueue::SongQueue;
use crate::app::search::Search;

mod navigator;
mod command;
mod songlist;
mod songqueue;
mod search;

#[derive(PartialEq)]
enum SelectedPanel {
    SongList,
    Nav,
    Queue,
    Search,
}

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
        
        // temporarily just doing a hardcoded search
        let mut songlist : SongList = SongList::new(self.songs.search_any("guilty"));
        songlist.order_items(SongOrder::Album);

        let mut songlist_state = ListState::default();
        songlist_state.select(Some(songlist.get_selection() as usize));

        let mut songqueue : SongQueue = SongQueue::new();
        let mut songqueue_state = ListState::default();
        songqueue_state.select(None);

        let mut navigator : Navigator = Navigator::new();
        let mut navigator_state = ListState::default();
        songqueue_state.select(None);

        navigator.fill_category(0, &mut self.songs.get_table(songdb::Table::Album).unwrap());
        navigator.fill_category(1, &mut self.songs.get_table(songdb::Table::Artist).unwrap());
        navigator.fill_category(2, &mut self.songs.get_table(songdb::Table::Genre).unwrap());

        let mut searchbar : Search = Search::new();

        let mut panel = SelectedPanel::SongList;

        loop {
            // read input
            let curr_panel : &mut dyn command::Command = match panel {
                SelectedPanel::SongList => &mut songlist,
                SelectedPanel::Queue => &mut songqueue,
                SelectedPanel::Nav => &mut navigator,
                SelectedPanel::Search => &mut searchbar,
                _ => &mut songlist,
            };

            let mut response : Option<Response> = None;
            match crossterm::event::poll(Duration::new(0,10000)) {
                Ok(true) => {
                    match crossterm::event::read()? {
                        Event::Key(event) => {
                            match event.modifiers {
                                KeyModifiers::CONTROL => {
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
                                        KeyCode::Char('s') => {
                                            panel = SelectedPanel::Search;
                                        }
                                        _ => {}
                                    }
                                },
                                KeyModifiers::NONE => {
                                    // command 
                                    let command : command::Event = match event.code {
                                        KeyCode::Esc => {break;}, // breaks out of loop
                                        KeyCode::Up => command::Event::Up,
                                        KeyCode::Down => command::Event::Down,
                                        KeyCode::Enter => command::Event::Accept,
                                        KeyCode::Left => command::Event::Left,
                                        KeyCode::Right => command::Event::Right,
                                        KeyCode::Backspace => command::Event::Back,
                                        KeyCode::Char(c) => command::Event::Char(c),
                                        _ => command::Event::Nothing, // else do nothing
                                    };

                                    response = curr_panel.command(&command);

                                    match event.code {
                                        KeyCode::Tab => {
                                            panel = SelectedPanel::Nav;
                                        },
                                        _ => {}
                                    }
                                }
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
                        },
                        Response::QueueSong(s) => {
                            songqueue.push(s);
                        },
                        Response::StopSong => {
                            let _ = self.player.stop();
                        },
                        Response::Query(v) => {
                            songlist = SongList::new(self.songs.search_query(&v));
                            songlist.order_items(SongOrder::Album);
                        },
                        Response::QueryAny(s) => {
                            songlist = SongList::new(self.songs.search_any(&s));
                            songlist.order_items(SongOrder::Album);
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
            let mut nav_selection : u32 = 1;
            for i in 0..navigator.get_selection().0 as usize {
                nav_selection += 1;
                nav_selection += navigator.items[i].2.len() as u32;
            }
            match navigator.get_selection().1 {
                Some(v) => {nav_selection += v;},
                None => {}, 
            }
            navigator_state.select(Some(nav_selection as usize));

            // check if player is done with song, play next if there is one
            if self.player.is_song_finished() {
                songqueue.pop_currently_playing();
                match songqueue.get_currently_playing_song() {
                    Some(s) => { let _ = self.player.play(&s.path[..]); }
                    None => {},
                }
            }

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
                    .title("block")
                    .borders(Borders::ALL);

                let list = song_list_to_tui_list(&songlist.items, panel == SelectedPanel::SongList);
                let queue = queue_to_tui_list(&songqueue, panel == SelectedPanel::Queue);

                f.render_stateful_widget(queue, right_top_chunk, &mut songqueue_state);
                f.render_widget(block.clone(), right_bottom_chunk);
                f.render_stateful_widget(list, center_chunk, &mut songlist_state);
                // f.render_widget(block.clone(), center_top_chunk);
                f.render_stateful_widget(nav_to_tui_list(&navigator, panel == SelectedPanel::Nav), left_chunk, &mut navigator_state);
                let current_song = songqueue.get_currently_playing_song();
                draw_song_detail(f, bottom_chunk, &self.player, &current_song);
                draw_search(f, center_top_chunk, &searchbar.query[..], panel == SelectedPanel::Search);
            })?;

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

fn song_list_to_tui_list(song_list : &Vec<Song>, selected: bool) -> List {
    // let mut song_list = self.songs.search_all();
    // song_list.sort_by(|a,b| a.album.cmp(&b.album));
    let item_list : Vec<ListItem> = song_list.iter().map(|x| ListItem::new(x.to_string())).collect();
    let color = if selected { Color::Yellow } else { Color::White };
    let list = List::new(item_list)
        .block(Block::default().title("songs").borders(Borders::ALL).border_style(Style::default().fg(color)))
        .style(Style::default().fg(Color::White))
        .highlight_symbol(">>");
    return list;
}

fn nav_to_tui_list(nav: &navigator::Navigator, selected: bool) -> List {
    let mut item_list : Vec<ListItem> = Vec::new();
    for i in &nav.items {
        item_list.push(ListItem::new(i.0.name.clone()));
        for j in &i.2 {
            let s = format!("- {}", j.clone());
            item_list.push(ListItem::new(s));
        }
    }
    // item_list.push(ListItem::new("something"));
    let color = if selected { Color::Yellow } else { Color::White };
    let list = List::new(item_list)
        .block(Block::default().title("nav").borders(Borders::ALL).border_style(Style::default().fg(color)))
        .style(Style::default().fg(Color::White))
        .highlight_symbol(">>");
    return list;
}

fn queue_to_tui_list(q : &songqueue::SongQueue, selected: bool) -> List {
    let mut item_list : Vec<ListItem> = q.queue.iter().map(|x| ListItem::new(x.to_string())).collect();
    match q.get_currently_playing() {
        Some(v) => { 
            let playing_item = item_list[v as usize].clone().style(Style::default().fg(Color::Green)); 
            item_list[v as usize] = playing_item;
        }
        None => {},
    }
    let color = if selected { Color::Yellow } else { Color::White };
    let list = List::new(item_list)
        .block(Block::default().title("song queue").borders(Borders::ALL).border_style(Style::default().fg(color)))
        .style(Style::default().fg(Color::White))
        .highlight_symbol(">>");
    return list;
}

fn song_detail(player : &player::Player) -> LineGauge {
    let time_left = player.get_time_left();
    let duration = player.get_song_duration();
    let mut fraction_played = (1.0 - time_left / duration).clamp(0.0,1.0);
    if fraction_played.is_nan() {
        fraction_played = 0.0;
    }
    let gauge = LineGauge::default()
        .block(Block::default().title("Progress"))
        .gauge_style(Style::default().fg(Color::White).bg(Color::Black).add_modifier(Modifier::ITALIC))
        .ratio(fraction_played);
    // let gauge = Paragraph::new(Text::from(fraction_played.to_string()));
    return gauge;
}

fn draw_song_detail(
    f : &mut Frame<CrosstermBackend<std::io::Stdout>>, 
    rect : Rect,
    player : &player::Player,
    song: &Option<Song>) {
    // render container
    f.render_widget(Block::default().borders(Borders::ALL).title("song details"),rect);

    // split container
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
                     Constraint::Min(1),
                     Constraint::Min(1),
                     Constraint::Min(1),
                     Constraint::Min(1),
                     Constraint::Percentage(100)
        ].as_ref())
        .split(rect);

    // render song info
    let song_name : &str = match song {
        Some(s) => &s.title[..],
        None => "no song atm",
    };
    let song_name_paragraph = Paragraph::new(Text::from(song_name));
    f.render_widget(song_name_paragraph, chunks[0]);
    let song_album : &str = match song {
        Some(s) => &s.album[..],
        None => "",
    };
    let song_album_paragraph = Paragraph::new(Text::from(song_album));
    f.render_widget(song_album_paragraph, chunks[1]);
    let song_artist : &str = match song {
        Some(s) => &s.artist[..],
        None => "",
    };
    let song_artist_paragraph = Paragraph::new(Text::from(song_artist));
    f.render_widget(song_artist_paragraph, chunks[2]);

    // render song progress
    f.render_widget(song_detail(player), chunks[4]);
}

fn draw_search(
    f : &mut Frame<CrosstermBackend<std::io::Stdout>>, 
    rect : Rect,
    query: &str,
    selected: bool) {
    let color = if selected { Color::Yellow } else { Color::White };
    let search_paragraph = Paragraph::new(Text::from(query))
        .block(Block::default().title("search").borders(Borders::ALL).border_style(Style::default().fg(color)));
    f.render_widget(search_paragraph, rect);
}

pub fn create(songdb: SongDB) -> App {
    let player = player::new();
    let app = App {
        songs: songdb,
        player
    };
    return app;
}
