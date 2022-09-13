mod songdb;
mod player;

fn main() {
    let args : Vec<String> = std::env::args().collect();
    println!("Hello, I'm making rumu!");

    // open database
    let db = match songdb::open("test.db") {
        Ok(d) => d,
        Err(_) => {
            println!("woops"); 
            return;
        },
    };
    
    // search songs 
    let songs = songdb::get_meta_dir(&args[1]);
    println!("{}", songs.len());
    for song in songs {
        let already_there = match db.check_change(&song.title, &song.album, true, false) {
            Some(v) => v,
            None => false
        };
        println!("exists in db? {}", already_there);
        if !already_there {
            let _ = db.add(&song);
        }
    }

    // look through database and remove any songs that don't exist in fs
    /*
    match db.prune_db() {
        Ok(_) => (),
        Err(e) => println!("{}",e),
    }; 
    */

    /*
    let player = player::create(db);
    let _ = player.start();
    */
}
