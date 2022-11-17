use std::collections::VecDeque;

use crate::app::command::{Event,Command};
use crate::song::Song;

pub struct SongQueue {
    pub queue: VecDeque<Song>,
    selection: Option<u32>,
    selected_song: Option<Song>,
}

impl Command for SongQueue {
    fn command(&mut self, event: &Event) {
        match event {
            Event::Up =>    { self.select_up(); },
            Event::Down =>  { self.select_down(); },
            Event::Back =>  { 
                self.selection.and_then(|v| {
                    self.remove_song(v as usize);
                    return Some(());
                });
            },
            Event::Left =>  { self.swap_up(); }
            Event::Right =>  { self.swap_down(); }
            Event::Accept => { self.select(); }
            _ => {},
        }
    }
}

impl SongQueue {
    pub fn get_selected_song(&self) -> Option<Song> {
        return match self.selection {
            Some(x) => {
                Some(self.queue[x as usize].clone())
            },
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
                    new = self.queue.len() as u32;
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

    pub fn select(&mut self) {
        self.selected_song = self.selection.and_then(|v| {
            return self.queue.get(v as usize).and_then(|v| {
                return Some(v.clone());
            });
        });
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

    pub fn new() -> SongQueue {
        let q = SongQueue {
            queue: VecDeque::new(),
            selection: None,
            selected_song: None,
        };
        return q;
    }
}
