mod songdb;

fn main() {
    println!("Hello, I'm making rumu!");
    let song1 = songdb::get_meta("/home/enrique/nextcloud/music/collection/GuiltyGearStrive/Requiem.ogg").unwrap();
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
    db.add(song1);
    db.add(song2);
}
