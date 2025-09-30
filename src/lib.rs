
mod models;

pub use crate::models::*;

use metaflac::Tag;
use std::ffi::OsString;
use std::fmt;
use std::fs;
use std::fs::File;
use std::path::PathBuf;


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
pub fn parse_single_album(album: Album) -> Result<String, &'static str>{

    let mut tracklist = String::from("Треклист:\n");
    let mut summary_duration = 0 as u64;
    for track in album.tracks{
        let mut popped_track = track.name.clone();
        popped_track.replace_range(popped_track.find(".flac").unwrap()..popped_track.len(), "");

        tracklist.push_str(popped_track.as_str());
        tracklist.push_str("\n");
        summary_duration += track.duration;
    }
    let mut track_list_to_push = tracklist.clone();
    track_list_to_push.pop();

    let duration = time_pars(summary_duration);

    let mut result = String::from(r#"[spoiler="YEAR - ALBUMNAME"]"#);

    result.push_str("\n[img=right]FASTPIC[/img]");

    result.push_str("\n[b]Продолжительность[/b]:");

    for dur in duration{
        result.push_str(&dur.to_string());
        result.push_str(":");
    }
    // deleting the last ":" char

    result.pop();

    result.push_str("\n");

    result.push_str(track_list_to_push.as_str());

    result.push_str("\n[/pre][/spoiler]");

    //let result= String::from(summary_duration.to_string()+"\n"+tracklist.as_str());

    Ok(result)
    }



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

pub fn flac_duration(p: &PathBuf) -> Option<u64> {
    let tag = match Tag::read_from_path(p) {
        Ok(t) => t,
        Err(e) => {
            println!("Unable to read from path {}:", e);
            return None;
        }
    };
    let stream_info = tag.get_streaminfo()?;
    let nb_sec = stream_info.total_samples / stream_info.sample_rate as u64;
    return Some(nb_sec);
}
fn time_pars(mut sec: u64) -> Vec<String> {
    let mut h = String::from("00");
    let mut min = String::from("00");
    if sec > 60 * 60 {
        if sec / (60 * 60) < 10 {
            h = String::from({ "0" });
            h.push_str(&(sec / (60 * 60)).to_string())
        } else {
            h = String::from({ &(sec / (60 * 60)).to_string() });
        }
        sec = sec / (60);
    }
    if sec > 60 {
        if sec / 60 < 10 {
            min = String::from({ "0" });
            min.push_str(&(sec / (60)).to_string())
        } else {
            min = String::from({ &((sec / (60)).to_string()) });
        }
        sec = sec % 60;
    }
    Vec::from([h, min, sec.to_string()])
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn duration_test() {
//         let duration_test = flac_duration(&PathBuf::from(
//             r"D:\Music\[幻覚アリア] Hallucination Aria\[幻覚アリア] 幻覚アリア\01 Prologue-hallucination-.flac",
//         ));
//         assert_eq!(duration_test.unwrap(), 43 as u64);
//     }
//     #[test]
//     fn not_multi_album_checking() {
//         assert_eq!(
//             Config {
//                 file_path: PathBuf::from(r"D:\Music\Sally (サリー)\2011 - Sally")
//             }
//                 .is_discography(),
//             false
//         );
//     }
//     #[test]
//     fn multi_album_checking() {
//         assert_eq!(
//             Config {
//                 file_path: PathBuf::from(r"D:\Music\Sally (サリー)")
//             }
//                 .is_discography(),
//             true
//         );
//     }
//     #[test]
//     fn getting_album() {
//         assert_eq!(
//             Album {
//                 name: String::from("Album name"),
//                 tracks: Vec::from([Track {
//                     name: String::from("ded.flac"),
//                     duration: 256
//                 }])
//             },
//             get_album(PathBuf::from(r"D:\Music\testebat")).unwrap()
//         );
//     }
//     #[test]
//     fn getting_empty_album() {
//         let f = match (get_album(PathBuf::from(r"D:\Music\testebat\nonmusicfolder"))) {
//             Ok(T) => "asd",
//             Err(E) => E,
//         };
//         assert_eq!("No tracks in directory", f);
//     }
// }
//
