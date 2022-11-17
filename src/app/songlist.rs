use std::cmp::Ordering;

use crate::app::command::{Event,Command};
use crate::song;
use crate::song::Song;

pub enum SongOrder {
    TrackNum,
    Album,
    Artist,
    Title,
}

pub struct SongList {
    pub items: Vec<Song>,
    selection: i32,
}

impl Command for SongList {
    fn command(&mut self, event: &Event) {
        match event {
            Event::Up => {
                self.selection -= 1;
                if self.selection < 0 {
                    self.selection = self.items.len() as i32 - 1;
                }
            },
            Event::Down => {
                self.selection += 1;
                if self.selection >= self.items.len() as i32 {
                    self.selection = 0;
                }
            },
            Event::Left => {
                self.selection -= 10;
                if self.selection < 0 {
                    self.selection = 0;
                }
            },
            Event::Right => {
                self.selection += 10;
                if self.selection >= self.items.len() as i32 {
                    self.selection = self.items.len() as i32 - 1;
                }
            }
            _ => {},
        }
    }
}

impl SongList {
    pub fn replace_items(&mut self, songs: Vec<Song>) {
        self.items = songs;
        self.selection = 0;
    }

    pub fn get_items(&mut self) -> Vec<Song> {
        return self.items.clone();
    }

    pub fn get_selection(&self) -> i32 {
        return self.selection;
    }

    pub fn order_items(&mut self, order: SongOrder) {
        match order {
            SongOrder::Album => {
                self.items.sort_by(|a,b| {
                    let mut track_result = SongList::song_cmp_album(a,b).unwrap();
                    if track_result == Ordering::Equal {
                        track_result = SongList::song_cmp_track(a,b).unwrap();
                    }
                    return track_result;
                })
            },
            _ => {},
        }
    }

    fn song_cmp_track(a: &Song, b: &Song) -> Option<Ordering> {
        return a.track_num.partial_cmp(&b.track_num);
    }

    fn song_cmp_album(a: &Song, b: &Song) -> Option<Ordering> {
        return a.album.partial_cmp(&b.album);
    }

    pub fn new(items: Vec<Song>) -> SongList {
        let songlist = SongList {
            items,
            selection: 0
        };
        return songlist;
    }
}
