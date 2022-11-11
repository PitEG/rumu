use crate::app::command::{Event,Command};

struct Navigator {
    categories: Vec<String>,
    opened_categories: Vec<bool>,
    sub_categories: Vec<Vec<String>>,
    selection: (Option<u32>,Option<u32>),
    search_query: Option<String>,
}

impl Command for Navigator {
    fn command(event: &Event) {
        match event {
            _ => {},
        }
    }
}

impl Navigator {
    fn get_search(&mut self) -> Option<String> {
        let search = match &self.search_query {
            Some(x) => Some(x.clone()),
            None => None,
        };
        self.search_query = None;
        return search;
    }

    fn fill_categories(&mut self,  categories: Vec<String>, sub_categories : Vec<Vec<String>>) {
        self.categories = categories;
        self.sub_categories = sub_categories;
    }
}
