use std::process::Command;
use std::{io,fs};
use json;
use sqlite;
use walkdir::WalkDir;

use crate::song::Song;
use crate::song;

pub mod query;
pub use self::query::Query;

pub fn get_meta(filepath: &str) -> Result<Song, io::Error> {
    // let comm = Command::new("ls").args([".","src"]).output().expect("lala");
    // println!("{}",String::from_utf8_lossy(&comm.stdout));
    // ffprobe -loglevel 0 -print_format json -show_format -show_streams [filepath]
    let command = Command::new("mediainfo")
        .args([
            "--Output=JSON",
            &filepath,
        ])
        .output()
        .expect("failed to run mediainfo");
    // let _ = String::from_utf8_lossy(&command.stderr);
    let output = String::from_utf8_lossy(&command.stdout);
    if !command.status.success() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "cannot ffprobe file"));
    }

    // parsing output
    let parsed = json::parse(&output).unwrap();
    let tags = &parsed["media"]["track"][0];
    // let stream = &parsed["streams"][0];
    
    if tags["Format"] != "Ogg" {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "not a vorbis file"));
    }
    
    let title = match tags["Title"].as_str() {
        Some(s) => String::from(s), None => String::from("unknown album")
    };
    let album = match tags["Album"].as_str() {
        Some(s) => String::from(s), None => String::from("unknown album")
    };
    let artist = match tags["Performer"].as_str() {
        Some(s) => String::from(s), None => String::from("unknown artist"),
    };
    let genre = match tags["Genre"] .as_str() {
        Some(s) => String::from(s), None => String::from("unknown genre")
    };
    let lyrics = match tags["Lyrics"] .as_str() {
        Some(s) => String::from(s), None => String::from("no lyrics")
    };
    let duration = match String::from(tags["Duration"].as_str().unwrap_or("0")).parse::<f64>() {
        Ok(i) => i,
        Err(_) => -1.0,
    };
    let year = match tags["Recorded_Date"].as_str() {
        Some(s) => match s.parse::<i64>() { Ok(i) => i, Err(_) => -1 },
        None => -1
    };
    let track_num = match tags["Track_Position"].as_str() {
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
        // hash: song_hash(&filepath)?, // expensive, do it only when needed 
        hash: String::from(""),
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

pub enum Table {
    Title,
    Album,
    TrackNum,
    Artist,
    Genre,
    Duration,
    Year,
    Hash,
    Path,
    Size,
}

impl ToString for Table {
    // The string is the actual name of the corresponding table in the database
    fn to_string(&self) -> String {
        return match self {
            Table::Title    => String::from("title"),
            Table::Album    => String::from("album"),
            Table::TrackNum => String::from("tracknumber"),
            Table::Artist   => String::from("artist"),
            Table::Genre    => String::from("genre"),
            Table::Duration => String::from("duration"),
            Table::Year     => String::from("year"),
            Table::Hash     => String::from("hash"),
            Table::Path     => String::from("path"),
            Table::Size     => String::from("size"),
        }
    }
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
        let mut statement = self.connection.prepare("insert into song values (:title,:album,:tracknum,:artist,:genre,:duration,:year,:path,:hash,:size)")?;
        statement.bind_by_name(":title", &song.title[..])?;
        statement.bind_by_name(":album", &song.album[..])?;
        statement.bind_by_name(":tracknum", song.track_num)?;
        statement.bind_by_name(":artist", &song.artist[..])?;
        statement.bind_by_name(":genre", &song.genre[..])?;
        statement.bind_by_name(":duration", song.duration)?;
        statement.bind_by_name(":year", song.year)?;
        // statement.bind_by_name(":hash", &song.hash[..])?;
        let hash = song::song_hash(&song.path[..]).ok().unwrap_or(String::from("")); // not safe
        statement.bind_by_name(":hash", &hash[..])?;
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
        let mut statement = self.connection.prepare("update song set TrackNumber = :tracknum, Artist = :artist, Genre = :genre, Duration = :duration Year = :year, Path = :path, Version = :hash, Size = :size where Title = :title and Album = :album")?;
        statement.bind_by_name(":title", &title[..])?;
        statement.bind_by_name(":album", &album[..])?;
        statement.bind_by_name(":tracknum", song.track_num)?;
        statement.bind_by_name(":artist", &song.artist[..])?;
        statement.bind_by_name(":genre", &song.genre[..])?;
        statement.bind_by_name(":duration", song.duration)?;
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

    pub fn get_meta(&self, title: &str, album: &str) -> Option<Song> {
        let mut statement = self.connection.prepare("select * from song where Title = :title and Album = :album").ok()?;
        statement.bind_by_name(":title", &title[..]).ok()?;
        statement.bind_by_name(":album", &album[..]).ok()?;

        return match self.query(&mut statement) {
            Some(x) => {
                if x.len() > 0 {
                    Some(x[0].clone())
                }
                else {
                    None
                }
            },
            None => None
        }
    }

    pub fn search_all(&self) -> Vec<Song> {
        let mut statement = match self.connection.prepare("select * from song").ok() {
            Some(x) => x,
            None => {return Vec::new()}
        };

        return match self.query(&mut statement) {
            Some(x) => x,
            None => {return Vec::new()}
        }
    }

    pub fn search_any(&self, s : &str) -> Vec<Song> {
        let s_any = format!("%{}%", s); //hmmmmmmmmmmmmmmmmmm this might just be gimmicky to bypass
                                        //prepare sanitation... maybe
        let mut statement = match self.connection.prepare(
            "select * from song where album like :album or title like :title or artist like :artist")
            .ok() {
                Some(x) => x,
                None => {return Vec::new()}
            };
        statement.bind_by_name(":album", &s_any[..]).ok();
        statement.bind_by_name(":title", &s_any[..]).ok();
        statement.bind_by_name(":artist", &s_any[..]).ok();
        return match self.query(&mut statement) {
            Some(x) => x,
            None => {return Vec::new()}
        }
    }

    pub fn search_query(&self, q : &Query) -> Vec<Song> {
        let mut statement = match self.connection.prepare(
            "select * from song where album like :album or title like :title or artist like :artist")
            .ok() {
                Some(x) => x,
                None => {return Vec::new()}
            };
        match &q.album { Some(v) => {statement.bind_by_name(":album", &v[..]).ok();}, None => {}}
        match &q.title { Some(v) => {statement.bind_by_name(":title", &v[..]).ok();}, None => {}}
        match &q.artist { Some(v) => {statement.bind_by_name(":artist", &v[..]).ok();}, None => {}}
        return match self.query(&mut statement) {
            Some(x) => x,
            None => {return Vec::new()}
        }
    }

    fn query(&self, statement : &mut sqlite::Statement) -> Option<Vec<Song>> {
        let mut song_list : Vec<Song> = Vec::new();
        while let sqlite::State::Row = statement.next().ok()? {
           let title = statement.read::<String>(0).ok()?; 
           let album = statement.read::<String>(1).ok()?; 
           let track_num = statement.read::<i64>(2).ok()?; 
           let artist = statement.read::<String>(3).ok()?; 
           let genre = statement.read::<String>(4).ok()?; 
           let duration = statement.read::<f64>(5).ok()?;
           let year = statement.read::<i64>(6).ok()?; 
           let path = statement.read::<String>(7).ok()?; 
           let hash = statement.read::<String>(8).ok()?; 
           let size = statement.read::<i64>(9).ok()?; 
           let song = Song{
               title,
               album,
               artist,
               genre,
               year, 
               track_num,
               duration,
               path,
               lyrics: String::from("placeholder"), // currently not querying in this function 
               hash,
               size,
           };
           song_list.push(song);
        }
        return Some(song_list);
    }

    pub fn get_table(&self, table : Table) -> Result<Vec<String>,sqlite::Error> {
        let mut results : Vec<String> = Vec::new();
        let mut statement = self.connection.prepare(format!("select distinct {} from song", table.to_string()))?;
        while let sqlite::State::Row = statement.next()? {
            let value = statement.read::<String>(0)?;
            results.push(value);
        }
        
        return Ok(results);
    }

    // checks if file of song in databse has changed
    // You can choose what to check (file size or checksum). If both are checked, size is checked
    // first.
    // returns true if the file size is inconsistent with db's entry of the song
    // returns true if the sha1 checksums are different
    // false if checksums are the same and the filesize is the same
    pub fn check_change(&self, title: &str, album: &str, check_size: bool, check_hash: bool) -> Option<bool> {
        let mut statement = self.connection.prepare("select Path,Size,Version from song where Title = :title and Album = :album").ok()?;
        statement.bind_by_name(":title", &title[..]).ok()?;
        statement.bind_by_name(":album", &album[..]).ok()?;
        statement.next().ok()?;

        let path = statement.read::<String>(0).ok()?;
        let size = statement.read::<i64>(1).ok()?;
        let hash = statement.read::<String>(2).ok()?;

        // check if sizes are the same
        if check_size {
            let actual_size = fs::metadata(&path[..]).ok()?.len();
            if size != (actual_size as i64) {
                return Some(false);
            }
        }

        // check if checksum is the same
        if check_hash {
            if song::song_hash(&path[..]).ok()? != hash {
                return Some(false);
            }
        }
        
        // if size are the same and checksum is the same, just assume it's the same
        return Some(true)
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
        create table if not exists song (Title TEXT, Album TEXT, TrackNumber INTEGER, Artist TEXT, Genre TEXT, Duration DECIMAL, Year INTEGER, Path TEXT, Version CHAR(16), Size INTEGER,
            CONSTRAINT PK_Song PRIMARY KEY (Title, Album));
        create table if not exists lyrics (Title TEXT NOT NULL, Album TEXT NOT NULL, Lyrics TEXT,
            FOREIGN KEY(Title) REFERENCES songs(Title),
            FOREIGN KEY(Album) REFERENCES songs(Album));
        "
    )?;
    return Ok(songdb);
}

