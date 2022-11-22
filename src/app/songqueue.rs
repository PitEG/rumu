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
        return match event {
            Event::Up =>    { self.select_up(); None },
            Event::Down =>  { self.select_down(); None },
            Event::Back =>  { 
                self.selection.and_then(|v| {
                    self.remove(v as usize);
                    None
                })
            },
            Event::Left =>  { self.swap_up(); None }
            Event::Right =>  { self.swap_down(); None }
            Event::Accept => { 
                match self.selection {
                    Some(v) => {
                        self.currently_playing = Some(v);
                        Some(Response::PlaySong(self.queue[v as usize].clone()))
                    },
                    None => None,
                }
            }
            _ => None,
        }
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
            None => {
                let mut selection = None;
                if self.queue.len() > 0 {
                    selection = Some(0);
                }
                selection
            }
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
            None => {
                let mut selection = None;
                if self.queue.len() > 0 {
                    selection = Some(0);
                }
                selection
            }
        }
    }

    pub fn swap_up(&mut self) {
        self.selection = match self.selection {
            Some(x) => {
                let mut new = x.wrapping_sub(1);
                if new >= self.queue.len() as u32 {
                    new = 0;
                }
                self.currently_playing = match self.currently_playing {
                    Some(_) => Some(new),
                    None => None,
                };
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
                    new = (self.queue.len() - 1) as u32;
                }
                self.currently_playing = match self.currently_playing {
                    Some(_) => Some(new),
                    None => None,
                };
                self.queue.swap(x as usize, new as usize);
                Some(new)
            },
            None => None,
        }
    }

    pub fn push(&mut self, song: Song) {
        self.queue.push_back(song);
    }

    pub fn remove(&mut self, idx: usize) {
        if (idx as u32) < self.queue.len() as u32 {
            self.queue.remove(idx as usize);
            self.currently_playing = match self.currently_playing {
                Some(v) => {
                    let mut result = Some(v);
                    if v > idx as u32 { result = Some(v - 1); }
                    result
                },
                None => None,
            };
            self.selection = match self.selection {
                Some(v) => {
                    let mut result = Some(v);
                    // case we deleted the last and only item
                    if self.queue.len() == 0 {
                        result = None
                    }
                    // case we deleted the item at the end
                    else if v == self.queue.len() as u32 {
                        result = Some(v - 1);
                    } 
                    result
                },
                None => None,
            }
        }
    }

    pub fn get_selection(&self) -> Option<u32> {
        return self.selection;
    }

    pub fn get_currently_playing(&self) -> Option<u32> {
        return self.currently_playing;
    }

    pub fn set_currently_playing(&mut self, idx: u32) {
        if idx >= self.queue.len() as u32 {
            return;
        }
        self.currently_playing = Some(idx);
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
