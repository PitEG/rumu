use mpv::{MpvHandler, MpvHandlerBuilder, EndFileReason};

pub struct Player {
    backend: MpvHandler,
}

pub fn new() -> Player {
    let mut handler = MpvHandlerBuilder::new().expect("something");
    handler.set_option("vid","no").expect("");
    let handler = handler.build().expect("");
    let player = Player {
        backend: handler
    };
    
    return player;
}

impl Player {

    fn command(&mut self, command : &mut [&str]) -> Result<(),&str> {
        match self.backend.command(command) {
            Ok(v) => Ok(v),
            Err(e) => {print!("{}",e); Err("command fail")},
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
                    // mpv::Event::Idle => { return true },
                    mpv::Event::EndFile(result) => { 
                        match result {
                            Ok(EndFileReason::MPV_END_FILE_REASON_EOF) => {
                                return true;
                            },
                            _ => return false,
                        }
                    },
                    _ => { return false },
                }
            },
            None => { return false },
        }
    }

    pub fn get_time_left(&self) -> f64 {
        match self.backend.get_property("time-remaining") {
            Ok(v) => v,
            Err(_) => 0.0,
        }
    }

    pub fn get_song_duration(&self) -> f64 {
        match self.backend.get_property("duration") {
            Ok(v) => v,
            Err(_) => 0.0,
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

