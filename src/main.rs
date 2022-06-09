mod songdb;
mod player;

fn main() {
    let args : Vec<String> = std::env::args().collect();
    println!("Hello, I'm making rumu!");
    let db = match songdb::open("test.db") {
        Ok(d) => d,
        Err(_) => {
            println!("woops"); 
            return;
        },
    };
    
    let songs = songdb::get_meta_dir(&args[1]);
    println!("{}", songs.len());
    for song in songs {
        let _ = db.add(&song);
    }

    match db.prune_db() {
        Ok(_) => (),
        Err(e) => println!("{}",e),
    }; 

    // let player = player::create(db);
    // let _ = player.start();
}
