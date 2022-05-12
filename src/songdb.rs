// use std::path::Path;
use std::process::Command;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use md5;
use json;
use sqlite;

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
}

impl Song {
    pub fn to_string(&self) -> String {
        return String::from(format!("{} - {} {}, {}; {}s", &self.track_num, &self.title, &self.album,&self.year,&self.duration));
    }

    pub fn path(&self) -> String {
        return String::clone(&self.path);
    }
}

// This is an expensive function. It takes a while to run.
fn song_hash(filepath: &str) -> Result<String, io::Error> {
    const BUFFER_SIZE: usize = 8192;
    let mut file = File::open(filepath)?;
    let mut buffer = [0; BUFFER_SIZE];
    let mut context = md5::Context::new();
    loop {
        // let count = reader.read(&mut buffer[..])?;
        let count = file.read(&mut buffer[..])?;
        context.consume(&buffer[..count]);
        if count == 0 { break };
    }
    let hash = context.compute();
    let hash_string = format!("{:x}",hash);
    return Ok(hash_string);
}

pub fn get_meta(filepath: &str) -> Result<Song, io::Error> {
    // let comm = Command::new("ls").args([".","src"]).output().expect("lala");
    // println!("{}",String::from_utf8_lossy(&comm.stdout));
    // ffprobe -loglevel 0 -print_format json -show_format -show_streams [filepath]
    let command = Command::new("ffprobe")
        .args([
            "-loglevel","0",
            "-print_format","json",
            "-show_format",
            "-show_streams",
            &filepath,
        ])
        .output()
        .expect("failed to run ffprobe");
    let _ = String::from_utf8_lossy(&command.stdout);
    let _ = String::from_utf8_lossy(&command.stderr);
    let output = String::from_utf8_lossy(&command.stdout);

    // parsing output
    let parsed = json::parse(&output).unwrap();
    // println!("{}",parsed);
    let tags = &parsed["streams"][0]["tags"];
    let stream = &parsed["streams"][0];
    // println!("tags:\n{}", &tags);
    
    let title = String::from(tags["TITLE"] .as_str().unwrap());
    let album = String::from(tags["ALBUM"] .as_str().unwrap());
    let artist = String::from(tags["ARTIST"].as_str().unwrap());
    let genre = String::from(tags["GENRE"] .as_str().unwrap());
    let lyrics = String::from(tags["LYRICS"] .as_str().unwrap());
    let duration = match String::from(stream["duration"].as_str().unwrap()).parse::<f64>() {
        Ok(i) => i,
        Err(_) => -1.0,
    };
    let year = match String::from(tags["DATE"].as_str().unwrap()).parse::<i64>() {
        Ok(i) => i,
        Err(_) => -1,
    };
    let track_num = match String::from(tags["track"].as_str().unwrap()).parse::<i64>() {
        Ok(i) => i,
        Err(_) => -1,
    };
    let song = Song{
        title,
        album,
        artist,
        genre,
        year, 
        track_num,
        duration,
        path: String::from(filepath),
        lyrics,
        hash: song_hash(&filepath)?,
    };

    return Ok(song);
}

pub struct SongDB {
    pub database_path: String,
    connection: sqlite::Connection,
}

// Song database
impl SongDB {
    // add a song to the database
    pub fn add(&self, song: Song) {
        // insert into song relation
        let mut statement = self.connection.prepare("insert into song values (:title,:album,:tracknum,:artist,:genre,:year,:hash)").unwrap();
        statement.bind_by_name(":title", &song.title[..]).unwrap();
        statement.bind_by_name(":album", &song.album[..]).unwrap();
        statement.bind_by_name(":tracknum", song.track_num).unwrap();
        statement.bind_by_name(":artist", &song.artist[..]).unwrap();
        statement.bind_by_name(":genre", &song.genre[..]).unwrap();
        statement.bind_by_name(":year", song.year).unwrap();
        statement.bind_by_name(":hash", &song.hash[..]).unwrap();
        let _ = statement.next(); // handle error later

        // insert into lyrics relation
        let mut statement = self.connection.prepare("insert into lyrics values (:title,:album,:lyrics)").unwrap();
        statement.bind_by_name(":title", &song.title[..]).unwrap();
        statement.bind_by_name(":album", &song.album[..]).unwrap();
        statement.bind_by_name(":lyrics", &song.lyrics[..]).unwrap();
        let _ = statement.next(); // handle error later
         
        return;
    }

    #[allow(dead_code)]
    pub fn remove(&self, title: &str, album: &str) {
        // remove from song relation
        let mut statement = self.connection.prepare("delete from song where title = :title and album = :album").unwrap();
        statement.bind_by_name(":title", &title[..]).unwrap();
        statement.bind_by_name(":album", &album[..]).unwrap();
        let _ = statement.next(); // handle error later

        // remove from lyrics relation
        let mut statement = self.connection.prepare("delete from lyrics where title = :title and album = :album").unwrap();
        statement.bind_by_name(":title", &title[..]).unwrap();
        statement.bind_by_name(":album", &album[..]).unwrap();
        let _ = statement.next(); // handle error later

        return;
    }

    /*
    pub fn update(&self, title: &str, album: &str, song: Song) {
        return;
    }
    */

    #[allow(dead_code)]
    pub fn search(&self) {
        return;
    }
}

// Open a song database file
pub fn open(db_path: &str) -> Result<SongDB,sqlite::Error> {
    let songdb = SongDB{
        database_path: String::from(db_path),
        connection: sqlite::open(db_path)?,
    };
    songdb.connection.execute(
        "
        create table if not exists song (Title TEXT, Album TEXT, TrackNumber INTEGER, Artist TEXT, Genre TEXT, Year INTEGER, Version CHAR(16),
            CONSTRAINT PK_Song PRIMARY KEY (Title, Album));
        create table if not exists lyrics (Title TEXT NOT NULL, Album TEXT NOT NULL, Lyrics TEXT,
            FOREIGN KEY(Title) REFERENCES songs(Title),
            FOREIGN KEY(Album) REFERENCES songs(Album));
        "
    )?;
    return Ok(songdb);
}

#[allow(dead_code)]
pub struct Search {
    pub curr_search: String,
}

