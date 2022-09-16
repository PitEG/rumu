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

    pub fn queue(&mut self, path: &str) -> Result<(),&str> {
        let mut command = ["loadfile", path, "append"];
        return self.command(&mut command);
    }

    pub fn play_queue(&mut self) -> Result<(),&str> {
        let mut command = ["playlist-next"];
        return self.command(&mut command);
    }

}

