use::std::fs;
use::std::path::PathBuf;

use super::album::{Album, get_album};

pub fn get_multi_album(path: PathBuf) -> Vec<Album> {
    let mut albums: Vec<Album> = Vec::new();

    let entries = fs::read_dir(path.clone()).unwrap();
    for entry in entries {
        let mut album_directory = entry.unwrap();
        if album_directory.file_type().unwrap().is_dir() {
            let mut album_path = path.clone();
            album_path.push(album_directory.file_name());
            match get_album(album_path.clone()) {
                Ok(album) => albums.push(album),
                Err(err) => {
                    println!("{err}")
                }
            };
        }
    }

    albums
}