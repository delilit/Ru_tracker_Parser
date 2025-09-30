// mod models;

// pub use crate::models::*;

// use metaflac::Tag;
// use std::ffi::OsString;
// use std::fmt;
// use std::fs;
// use std::fs::File;
// use std::path::PathBuf;
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
