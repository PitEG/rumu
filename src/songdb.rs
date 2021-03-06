use std::process::Command;
use std::{io,fs};
use std::io::prelude::*;
use std::fs::File;
use sha1::{Sha1,Digest};
use json;
use sqlite;
use walkdir::WalkDir;

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
}

// This is an expensive function. It takes a while to run.
// it hashes the whole file appended with the file path
fn song_hash(filepath: &str) -> Result<String, io::Error> {
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
    // let _ = String::from_utf8_lossy(&command.stderr);
    let output = String::from_utf8_lossy(&command.stdout);
    if !command.status.success() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "cannot ffprobe file"));
    }

    // parsing output
    let parsed = json::parse(&output).unwrap();
    // println!("{}",parsed);
    let tags = &parsed["streams"][0]["tags"];
    let stream = &parsed["streams"][0];
    // println!("tags:\n{}", &tags);
    
    if stream["codec_name"] != "vorbis" {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "not a vorbis file"));
    }
    
    let title = match tags["TITLE"].as_str() {
        Some(s) => String::from(s), None => String::from("unknown album")
    };
    let album = match tags["ALBUM"].as_str() {
        Some(s) => String::from(s), None => String::from("unknown album")
    };
    let artist = match tags["ARTIST"].as_str() {
        Some(s) => String::from(s), None => String::from("unknown artist"),
    };
    let genre = match tags["GENRE"] .as_str() {
        Some(s) => String::from(s), None => String::from("unknown genre")
    };
    let lyrics = match tags["LYRICS"] .as_str() {
        Some(s) => String::from(s), None => String::from("no lyrics")
    };
    let duration = match String::from(stream["duration"].as_str().unwrap()).parse::<f64>() {
        Ok(i) => i,
        Err(_) => -1.0,
    };
    let year = match tags["DATE"].as_str() {
        Some(s) => match s.parse::<i64>() { Ok(i) => i, Err(_) => -1 },
        None => -1
    };
    let track_num = match tags["track"].as_str() {
        Some(s) => match s.parse::<i64>() { Ok(i) => i, Err(_) => -1 },
        None => -1
    };
    let size = fs::metadata(filepath)?.len() as i64;
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
        size,
    };

    return Ok(song);
}

pub fn get_meta_dir(dir: &str) -> Vec<Song> {
    // walk through every file in this directory and its subdirectories
    let mut songs : Vec<Song> = vec![];
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        // println!("ffprobing: {}",entry.path().display());
        // get_meta() is an expensive function
        let meta : Song = match get_meta(entry.path().to_str().unwrap()) {
            Ok(m) => {
                println!("successful!");
                m
            },
            Err(_) => continue, 
        };
        songs.push(meta);
    }

    return songs;
}

pub struct SongDB {
    pub database_path: String,
    connection: sqlite::Connection,
}

// Song database
impl SongDB {
    // add a song to the database
    pub fn add(&self, song: &Song) -> Result<(),sqlite::Error>{
        // insert into song relation
        let mut statement = self.connection.prepare("insert into song values (:title,:album,:tracknum,:artist,:genre,:year,:path,:hash,:size)")?;
        statement.bind_by_name(":title", &song.title[..])?;
        statement.bind_by_name(":album", &song.album[..])?;
        statement.bind_by_name(":tracknum", song.track_num)?;
        statement.bind_by_name(":artist", &song.artist[..])?;
        statement.bind_by_name(":genre", &song.genre[..])?;
        statement.bind_by_name(":year", song.year)?;
        statement.bind_by_name(":hash", &song.hash[..])?;
        statement.bind_by_name(":path", &song.path[..])?;
        statement.bind_by_name(":size", song.size)?;
        let _ = statement.next(); // handle error later

        // insert into lyrics relation
        let mut statement = self.connection.prepare("insert into lyrics values (:title,:album,:lyrics)")?;
        statement.bind_by_name(":title", &song.title[..])?;
        statement.bind_by_name(":album", &song.album[..])?;
        statement.bind_by_name(":lyrics", &song.lyrics[..])?;
        let _ = statement.next(); // handle error later
         
        return Ok(());
    }

    pub fn remove(&self, title: &str, album: &str) -> Result<(),sqlite::Error> {
        // remove from song relation
        let mut statement = self.connection.prepare("delete from song where title = :title and album = :album")?;
        statement.bind_by_name(":title", &title[..])?;
        statement.bind_by_name(":album", &album[..])?;
        let _ = statement.next(); // handle error later

        // remove from lyrics relation
        let mut statement = self.connection.prepare("delete from lyrics where title = :title and album = :album")?;
        statement.bind_by_name(":title", &title[..])?;
        statement.bind_by_name(":album", &album[..])?;
        let _ = statement.next(); // handle error later

        return Ok(());
    }

    pub fn update(&self, title: &str, album: &str, song: &Song) -> Result<(),sqlite::Error>{
        // insert into song relation
        let mut statement = self.connection.prepare("update song set TrackNumber = :tracknum, Artist = :artist, Genre = :genre, Year = :year, Path = :path, Version = :hash, Size = :size where Title = :title and Album = :album")?;
        statement.bind_by_name(":title", &title[..])?;
        statement.bind_by_name(":album", &album[..])?;
        statement.bind_by_name(":tracknum", song.track_num)?;
        statement.bind_by_name(":artist", &song.artist[..])?;
        statement.bind_by_name(":genre", &song.genre[..])?;
        statement.bind_by_name(":year", song.year)?;
        statement.bind_by_name(":hash", &song.hash[..])?;
        statement.bind_by_name(":path", &song.path[..])?;
        statement.bind_by_name(":size", song.size)?;
        let _ = statement.next(); // handle error later

        // insert into lyrics relation
        let mut statement = self.connection.prepare("update lyrics set Lyrics = :lyrics where Title = :title and Album = :album")?;
        statement.bind_by_name(":title", &title[..])?;
        statement.bind_by_name(":album", &album[..])?;
        statement.bind_by_name(":lyrics", &song.lyrics[..])?;
        let _ = statement.next(); // handle error later
         
        return Ok(());
    }

    // go through the db and remove entries that don't exist in the fs
    // does not read if it's actually the same file
    pub fn prune_db(&self) -> Result<(),sqlite::Error> {
        // get a list paths that should have a song
        let mut songs : Vec<(String,String,String)> = vec![]; // Title,Album,Path
        
        // sql query
        let mut statement = self.connection.prepare("select Title,Album,Path from song")?;
        while let sqlite::State::Row = statement.next()? {
           let title = statement.read::<String>(0)?; 
           let album = statement.read::<String>(1)?; 
           let path = statement.read::<String>(2)?; 
           songs.push((title,album,path));
        }

        // check to see if each song path still exists
        let mut missing : Vec<(String,String)> = vec![];
        for song in songs {
            match fs::metadata(song.2) {
                Ok(_) => println!("it's there"),
                Err(_) => {
                    missing.push((song.0,song.1));
                    println!("not there");
                }
            };
        }

        println!("{}",missing.len());
        for song in missing {
            self.remove(&song.0[..], &song.1[..])?;
        }

        return Ok(());
    }

    // look through directory recursively and add song files that don't exist in db
    pub fn add_new_songs_dir(&self, dir: &str) {
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        }
    }

    pub fn get_meta(&self, title: &str, album: &str) -> Option<Song> {
        let mut statement = self.connection.prepare("select * from song where Title = :title and Album = :album").ok()?;
        statement.bind_by_name(":title", &title[..]).ok()?;
        statement.bind_by_name(":album", &album[..]).ok()?;

        while let sqlite::State::Row = statement.next().ok()? {
           let title = statement.read::<String>(0).ok()?; 
           let album = statement.read::<String>(1).ok()?; 
           let track_num = statement.read::<i64>(2).ok()?; 
           let artist = statement.read::<String>(3).ok()?; 
           let genre = statement.read::<String>(4).ok()?; 
           let year = statement.read::<i64>(5).ok()?; 
           let path = statement.read::<String>(6).ok()?; 
           let hash = statement.read::<String>(7).ok()?; 
           let size = statement.read::<i64>(8).ok()?; 
           let song = Song{
               title,
               album,
               artist,
               genre,
               year, 
               track_num,
               duration: 0., // currently not saved
               path,
               lyrics: String::from("placeholder"), // currently not querying in this function 
               hash,
               size,
           };
           return Some(song);
        }
        return None;
    }

    // checks if file of song in databse has changed
    // returns true if the file size is inconsistent with db's entry of the song
    // returns true if the sha1 checksums are different
    // false if checksums are the same and the filesize is the same
    pub fn check_change(&self, title: &str, album: &str) -> Option<bool> {
        let mut statement = self.connection.prepare("select Path,Size,Version from song where Title = :title and Album = :album").ok()?;
        statement.bind_by_name(":title", &title[..]).ok()?;
        statement.bind_by_name(":album", &album[..]).ok()?;
        statement.next().ok()?;

        let path = statement.read::<String>(0).ok()?;
        let size = statement.read::<i64>(1).ok()?;
        let hash = statement.read::<String>(2).ok()?;

        // check if sizes are the same
        let actual_size = fs::metadata(&path[..]).ok()?.len();
        if size != (actual_size as i64) {
            return Some(false);
        }

        // check if checksum is the same
        if song_hash(&path[..]).ok()? != hash {
            return Some(false);
        }
        
        // if size are the same and checksum is the same, just assume it's the same
        return Some(true)
    }

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
        create table if not exists song (Title TEXT, Album TEXT, TrackNumber INTEGER, Artist TEXT, Genre TEXT, Year INTEGER, Path TEXT, Version CHAR(16), Size INTEGER,
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

