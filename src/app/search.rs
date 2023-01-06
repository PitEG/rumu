use crate::app::command::{Event,Command,Response};
use std::cmp;

pub struct Search {
    pub query : String
}

impl Command for Search {
    fn command(&mut self, event: &Event) -> Option<Response> {
        match event {
            Event::Char(c) => {
                self.query.push(*c);
            },
            Event::Back => {
                let clamped_slice = match (self.query.len() as usize).checked_sub(1) {
                    Some(v) => v,
                    None => 0,
                };
                self.query = self.query[0..clamped_slice].to_string();
            },
            Event::Accept => {
                return Some(Response::QueryAny(self.query.clone()));
            }
            _ => {}
        }
        return None
    }
}

impl Search {
    pub fn new() -> Search {
        return Search {
            query: String::from("test search query"),
        }
    }
}
