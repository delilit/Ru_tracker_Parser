use super::track::Track;

use std::fmt;
use std::path::PathBuf;
use std::fs;

use crate::utils::duration_parser::*;

#[derive(PartialEq)]
pub struct Album {
    pub name: String,
    pub tracks: Vec<Track>,
}
//new commentary get
impl fmt::Debug for Album {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Track")
            .field("name", &self.name)
            .field("tracks", &self.tracks)
            .finish()
    }
}

pub fn get_album(path: PathBuf) -> Result<Album, &'static str> {
    let mut tracks: Vec<Track> = Vec::new();

    let entries = fs::read_dir(&path).unwrap();
    for mut entry in entries {
        let file_name = entry.unwrap().file_name();
        if file_name.len() > 5 {
            if file_name.to_str().unwrap().contains(".flac") {
                //println!("{}",&self.file_path.clone().into_os_string().as_os_str().to_str().unwrap());
                let mut track_path = &mut path.clone();
                track_path.push(file_name.clone());
                tracks.push(Track {
                    name: file_name.clone().into_string().unwrap(),
                    duration: flac_duration(track_path).unwrap(),
                })
            }
        }
    }
    if tracks.len() < 1 {
        Err("No tracks in directory")
    } else {
        Ok(Album {
            name: String::from("Album name"),
            tracks: tracks,
        })
    }
}