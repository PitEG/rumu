use mpv::{MpvHandler, MpvHandlerBuilder};

pub struct Player {
    backend: MpvHandler,
    queue: Vec<String>,
}

pub fn new() -> Player {
    let mut handler = MpvHandlerBuilder::new().expect("something");
    handler.set_option("vid","no").expect("");
    let handler = handler.build().expect("");
    let player = Player {
        backend: handler,
        queue: Vec::new(),
    };
    
    return player;
}

impl Player {

    fn command(&mut self, command : &mut [&str]) -> Result<(),&str> {
        match self.backend.command(command) {
            Ok(v) => Ok(v),
            Err(e) => {print!("{}",e); Err("queue fail")},
        }
    }

    pub fn play(&mut self, path: &str) -> Result<(),&str> {
        let mut command = ["loadfile", path, "append-play"];
        return self.command(&mut command);
    }

    pub fn stop(&mut self) -> Result<(),&str> {
        let mut command = ["stop"];
        return self.command(&mut command);
    }

    pub fn is_song_finished(&mut self) -> bool {
        match self.backend.wait_event(0.001) {
            Some(v) => {
                match v {
                    mpv::Event::Idle => { return true },
                    _ => { return false },
                }
            },
            None => { return false },
        }
    }

    // probably not use these two
    pub fn queue(&mut self, path: &str) -> Result<(),&str> {
        let mut command = ["loadfile", path, "append"];
        return self.command(&mut command);
    }

    pub fn play_queue(&mut self) -> Result<(),&str> {
        let mut command = ["playlist-next"];
        return self.command(&mut command);
    }
}

