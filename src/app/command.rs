use crate::song::Song;

pub enum Event {
    Char(char),
    Up,
    Down,
    Left,
    Right,
    Accept,
    Back,
    None,
}

pub struct Response {
    song : Option<Song>,
    query : Option<(String,String)>
}

pub trait Command {
    fn command(&mut self, event : &Event);
}
