use metaflac::Tag;
use::std::path::PathBuf;

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