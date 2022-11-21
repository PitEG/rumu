use std::collections::VecDeque;

use crate::app::command::{Event,Command,Response};
use crate::song::Song;

pub struct SongQueue {
    pub queue: VecDeque<Song>,
    selection: Option<u32>,
    currently_playing: Option<u32>,
}

impl Command for SongQueue {
    fn command(&mut self, event: &Event) -> Option<Response> {
        match event {
            Event::Up =>    { self.select_up(); },
            Event::Down =>  { self.select_down(); },
            Event::Back =>  { 
                self.selection.and_then(|v| {
                    self.remove_song(v as usize);
                    Some(())
                });
            },
            Event::Left =>  { self.swap_up(); }
            Event::Right =>  { self.swap_down(); }
            Event::Accept => { 
                match self.selection {
                    Some(v) => {
                        return Some(Response::PlaySong(self.queue[v as usize].clone()));
                    },
                    None => {},
                }
            }
            _ => {},
        }
        return None;
    }
}

impl SongQueue {
    pub fn get_selected_song(&self) -> Option<Song> {
        let i = match self.selection {
            Some(v) => v,
            None => { return None },
        };
        return match self.queue.get(i as usize) {
            Some(s) => Some(s.clone()),
            None => None,
        }
    }

    pub fn select_down(&mut self) {
        self.selection = match self.selection {
            Some(x) => {
                let mut new = x + 1;
                if new >= self.queue.len() as u32 {
                    new = 0;
                }
                Some(new)
            },
            None => Some(0),
        }
    }

    pub fn select_up(&mut self) {
        self.selection = match self.selection {
            Some(x) => {
                let mut new = x.wrapping_sub(1);
                if new >= self.queue.len() as u32 {
                    new = (self.queue.len() - 1) as u32;
                }
                Some(new)
            }
            None => Some(0),
        }
    }

    pub fn swap_up(&mut self) {
        self.selection = match self.selection {
            Some(x) => {
                let mut new = x.wrapping_sub(1);
                if new >= self.queue.len() as u32 {
                    new = 0;
                }
                self.queue.swap(x as usize, new as usize);
                Some(new)
            },
            None => None,
        }
    }

    pub fn swap_down(&mut self) {
        self.selection = match self.selection {
            Some(x) => {
                let mut new = x + 1;
                if new >= self.queue.len() as u32 {
                    new = self.queue.len() as u32;
                }
                self.queue.swap(x as usize, new as usize);
                Some(new)
            },
            None => None,
        }
    }

    pub fn remove_song(&mut self, idx: usize) {
        self.queue.remove(idx);
        self.selection = self.selection.and_then(|v| {
            let queue_size = self.queue.len() as u32;
            if v >= queue_size {
                return Some(queue_size - 1);
            }
            else {
                return Some(v);
            }
        });
    }

    pub fn push(&mut self, song: Song) {
        self.queue.push_back(song);
    }

    pub fn get_selection(&mut self) -> Option<u32> {
        return self.selection;
    }

    pub fn new() -> SongQueue {
        let q = SongQueue {
            queue: VecDeque::new(),
            selection: None,
            currently_playing: None,
        };
        return q;
    }
}
