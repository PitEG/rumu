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

pub trait Command {
    fn command(&mut self, event : &Event);
}
