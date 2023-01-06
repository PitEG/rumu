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
    Nothing,
}

pub enum Response {
    QueueSong(Song),
    PlaySong(Song),
    Query(Query),
    QueryAny(String),
    StopSong,
}

pub trait Command {
    fn command(&mut self, event : &Event) -> Option<Response>;
}
