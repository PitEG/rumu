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
    N,
}

pub enum Response {
    QueueSong(Song),
    PlaySong(Song),
    Query(Query),
}

pub trait Command {
    fn command(&mut self, event : &Event) -> Option<Response>;
}
