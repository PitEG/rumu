use std::cmp;
use crate::app::command::{Event,Command,Response};
use crate::songdb::Query;

pub struct Category {
    pub name: String,
    pub table: String,
}

pub struct Navigator {
    pub items: Vec<(Category,bool,Vec<String>)>,
    selection: (u32,Option<u32>),
}


impl Command for Navigator {
    fn command(&mut self, event: &Event) -> Option<Response> {
        // 
        // UP/DOWN: move selection
        //
        return match event {
            Event::Up => {self.back(); None },
            Event::Down => {self.next(); None },
            Event::Right => {self.next_category(); None },
            Event::Left => {self.back_category(); None },
            Event::Accept => Some(Response::Query(self.query())),
            _ => None,
        };
    }
}

impl Navigator {
    pub fn back(&mut self) {
        let cat_size = self.items.get(self.selection.0 as usize).unwrap().2.len();
        self.selection.1 = match self.selection.1 {
            Some(v) => Some(cmp::min(v.wrapping_sub(1), cat_size as u32 - 1)),
            None => Some(0),
        };
    }

    pub fn next(&mut self) {
        let cat_size = self.items.get(self.selection.0 as usize).unwrap().2.len();
        self.selection.1 = match self.selection.1 {
            Some(v) => Some((v + 1).clamp(0,cat_size as u32 - 1)),
            None => Some(0),
        };
    }

    pub fn back_category(&mut self) {
        self.selection.0 = cmp::min(self.selection.0.wrapping_sub(1), self.items.len() as u32 - 1);
        self.selection.1 = Some(0);
    }

    pub fn next_category(&mut self) {
        self.selection.0 = (self.selection.0 + 1).clamp(0, self.items.len() as u32 - 1);
        self.selection.1 = Some(0);
    }

    pub fn fill_category(&mut self, idx : usize, content : &mut Vec<String>) {
        if idx < self.items.len() {
            self.items[idx].2.append(content);
        }
    }

    fn query(&self) -> Query {
        let table = self.items[self.selection.0 as usize].0.table.clone();
        let mut query = Query {
            title:     None,
            album:     None,
            artist:    None,
            genre:     None,
            year:      None,
            track_num: None,
            duration:  None,
            path:      None,
            lyrics:    None,
            hash:      None,
        };
        let query_content = self.items[self.selection.0 as usize].2[self.selection.1.unwrap() as usize].clone();
        match &table[..] {
            "Album" => { query.album = Some(query_content); },
            "Artist" => { query.artist= Some(query_content); },
            "Genre" => { query.genre = Some(query_content); },
            _ => {},
        };
        return query;
    }

    pub fn get_selection(&self) -> (u32,Option<u32>) {
        return self.selection;
    }

    pub fn new() -> Navigator {
        let mut items: Vec<(Category,bool,Vec<String>)> = Vec::new();
        items.push((
                Category {
                    name: String::from("Album"),
                    table: String::from("Album"),
                },
                true,
                Vec::new()
                ));
        items.push((
                Category {
                    name: String::from("Artist"),
                    table: String::from("Artist"),
                },
                true,
                Vec::new()
                ));
        items.push((
                Category {
                    name: String::from("Genre"),
                    table: String::from("Genre"),
                },
                true,
                Vec::new()
                ));
        let selection: (u32,Option<u32>) = (0,None);
        let nav = Navigator {
            items,
            selection,
        };
        return nav;
    }
}
