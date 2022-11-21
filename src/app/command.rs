use crate::song::Song;
use crate::songdb::Query;

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

pub enum Response {
    Song(Song),
    Query(Query),
}

pub trait Command {
    fn command(&mut self, event : &Event);
}
