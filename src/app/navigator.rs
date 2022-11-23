use crate::app::command::{Event,Command,Response};

pub struct Category {
    pub name: String,
    pub table: String,
}

pub struct Navigator {
    pub items: Vec<(Category,bool,Vec<String>)>,
    selection: (i32,Option<i32>),
}


impl Command for Navigator {
    fn command(&mut self, event: &Event) -> Option<Response> {
        // 
        // UP/DOWN: move selection
        //
        return match event {
            Event::Up => {self.back(); None },
            Event::Down => {self.next(); None },
            _ => None,
        };
    }
}

impl Navigator {
    pub fn get_selection(&self) -> (u32,Option<u32>) {
        return (self.selection.0 as u32, 
                self.selection.1.and_then(|v| Some(v as u32)));
    }

    pub fn size_of_category(&self, pos : usize) -> i32 {
        return self.items[pos].2.len() as i32;
    }

    pub fn fill_category(&mut self, idx : usize, content : &mut Vec<String>) {
        if idx < self.items.len() {
            self.items[idx].2.append(content);
        }
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
        let selection: (i32,Option<i32>) = (0,None);
        let nav = Navigator {
            items,
            selection,
        };
        return nav;
    }
}
