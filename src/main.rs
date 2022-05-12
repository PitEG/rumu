mod songdb;
mod player;

fn main() {
    println!("Hello, I'm making rumu!");
    let mut song1 = songdb::get_meta("/home/enrique/nextcloud/music/collection/GuiltyGearStrive/Requiem.ogg").unwrap();
    let song2 = songdb::get_meta("/home/enrique/nextcloud/music/collection/GuiltyGearStrive/LoveTheSubhumanSelf.ogg").unwrap();
    println!("read the songs!");
    let db = match songdb::open("test.db") {
        Ok(d) => d,
        Err(_) => {
            println!("woops"); 
            return;
        },
    };
    println!("{}", song1.to_string());
    println!("path: {}", song1.path());
    db.add(&song1);
    db.add(&song2);

    song1.year = 1000;
    println!("{}", song1.to_string());

    db.remove(&song2.title.clone(), &song2.album.clone());
    db.update("Requiem", "Guilty Gear Strive OST Vocals", &song1);

    let player = player::create(db);
    let _ = player.start();
}
