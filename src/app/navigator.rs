use crate::app::command::{Event,Command};

struct Category {
    name: String,
    table: String,
}

pub struct Navigator {
    items: Vec<(Category,bool,Vec<String>)>,
    selection: (u32,Option<i32>),
    search_query: Option<String>,
}

impl Command for Navigator {
    fn command(&mut self, event: &Event) {
        // move up and down the subcategory
        self.selection.1 = match event {
            Event::Up => self.selection.1.and_then(|v| Some(v-1)),
            Event::Down => self.selection.1.and_then(|v| Some(v+1)),
            _ => self.selection.1,
        };
        // check if we leave subcategory and move if we do // SHADY
        self.selection.1 = self.selection.1.and_then(|v| {
            let subcategory_size = self.items[self.selection.0 as usize].2.len() as u32;
            if v < 0 { 
                self.selection.0 = std::cmp::min(subcategory_size-1,self.selection.0.overflowing_sub(1).0);
                return None; 
            }
            else if v >= subcategory_size as i32 { 
                self.selection.0 = (self.selection.0 + 1) % subcategory_size;
                return None; 
            }
            return Some(v);
        });
    }
}

impl Navigator {
    pub fn get_search(&mut self) -> Option<String> {
        let search = match &self.search_query {
            Some(x) => Some(x.clone()),
            None => None,
        };
        self.search_query = None;
        return search;
    }

    pub fn get_selection(&self) -> (u32,Option<u32>) {
        return self.selection;
    }
}
