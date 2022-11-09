use std::io;
use std::io::prelude::*;
use std::fs::File;
use sha1::{Sha1,Digest};

#[derive(Clone)]
pub struct Song {
    pub title: String,
    pub album: String,
    pub artist: String,
    pub genre: String,
    pub year: i64,
    pub track_num: i64,
    pub duration: f64, // in seconds
    pub path: String,
    pub lyrics: String,
    pub hash: String,
    pub size: i64,
}

impl Song {
    pub fn to_string(&self) -> String {
        return String::from(format!("{} - {} {}, {}; {}s", &self.track_num, &self.title, &self.album,&self.year,&self.duration));
    }

    pub fn hash(&mut self) -> Result<(), io::Error> {
        self.hash = song_hash(&self.path)?;
        return Ok(());
    }
}

// This is an expensive function. It takes a while to run.
// it hashes the whole file appended with the file path
pub fn song_hash(filepath: &str) -> Result<String, io::Error> {
    const BUFFER_SIZE: usize = 8192;
    let mut file = File::open(filepath)?;
    let mut buffer = [0; BUFFER_SIZE];
    let mut context = Sha1::new();
    loop {
        // let count = reader.read(&mut buffer[..])?;
        let count = file.read(&mut buffer[..])?;
        context.update(&buffer[..count]);
        if count == 0 { break };
    }
    context.update(&filepath[..]);
    let hash = context.finalize();
    let hash_string = format!("{:x}",hash);
    return Ok(hash_string);
}

