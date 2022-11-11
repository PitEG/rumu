pub enum Event {
    Char(char),
    Up,
    Down,
    Left,
    Right,
    Accept,
    Back,
}

pub trait Command {
    fn command(event : &Event);
}