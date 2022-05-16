mod songdb;
mod player;

fn main() {
    println!("Hello, I'm making rumu!");
    let db = match songdb::open("test.db") {
        Ok(d) => d,
        Err(_) => {
            println!("woops"); 
            return;
        },
    };
    
    let songs = songdb::get_meta_dir("/home/enrique/nextcloud/music");
    for song in songs {
        db.add(&song);
    }

    // let player = player::create(db);
    // let _ = player.start();
}
