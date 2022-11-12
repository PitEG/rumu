use crate::app::command::{Event,Command};

struct Category {
    name: String,
    table: String,
}

pub struct Navigator {
    items: Vec<(Category,bool,Vec<String>)>,
    selection: (i32,Option<i32>),
    search_query: Option<(String,String)>,
}


impl Command for Navigator {
    fn command(&mut self, event: &Event) {
        // 
        // UP/DOWN: move selection
        //
        match event {
            Event::Up => {self.back()},
            Event::Down => {self.next()},
            _ => {},
        };
    }
}

impl Navigator {
    pub fn get_search(&mut self) -> Option<(String,String)> {
        let search = match &self.search_query {
            Some(x) => Some((x.0.clone(),x.1.clone())),
            None => None,
        };
        self.search_query = None;
        return search;
    }

    pub fn get_selection(&self) -> (u32,Option<u32>) {
        return (self.selection.0 as u32, 
                self.selection.1.and_then(|v| Some(v as u32)));
    }

    fn size_of_category(&self, pos : usize) -> i32 {
        return self.items[pos].2.len() as i32;
    }

    fn next(&mut self) {
        self.selection.1 = match self.selection.1 {
            Some(v) => {
                // subcat -> subcat
                let mut result = Some(v + 1);
                // subcat -> cat
                if (v + 1) > self.size_of_category(self.selection.0 as usize) {
                    self.selection.0 += 1;
                    self.selection.0 %= self.items.len() as i32;
                    result = None;
                }
                result
            },
            None => {
                let mut result = None;
                // cat -> subcat
                if self.items[self.selection.0 as usize].1 { // if open
                    result = Some(0);
                }
                // cat -> cat
                else {
                    self.selection.0 += 1;
                    self.selection.0 %= self.items.len() as i32;
                }
                result
            },
        };
    }

    fn back(&mut self) {
        self.selection.1 = match self.selection.1 {
            Some(v) => {
                // subcat -> subcat
                let mut result = Some(v - 1);
                // subcat -> cat
                if (v - 1) < 0 {
                    self.selection.0 -= 1;
                    if self.selection.0 < 0 {
                        self.selection.0 = self.items.len() as i32;
                    }
                    result = None
                }
                result
            }
            None => {
                let mut result = None;
                self.selection.0 -= 1;
                if self.selection.0 < 0 {
                    self.selection.0 = self.items.len() as i32;
                }
                // cat -> subcat
                if self.items[self.selection.0 as usize].1 {
                    result = Some(self.size_of_category(self.selection.0 as usize) - 1);
                }
                // cat -> cat // default when the above if block doesn't run
                result
            },
        }
    }
}
