use metaflac::Tag;
use::std::path::PathBuf;
//for mp3 duration
use std::path::Path;
use mp3_duration;

use hound::WavReader;
use std::path::Path;

pub fn flac_duration(path: &PathBuf) -> Option<u64> {
    if path.extension().is_some_and(|ext| ext == "flac"){
       let tag = match Tag::read_from_path(p) {
        Ok(t) => t,
        Err(e) => {
            println!("Unable to read from path {}:", e);
            return None;
        }
        };
        let stream_info = tag.get_streaminfo()?;
        let duration = stream_info.total_samples / stream_info.sample_rate as u64;
        return Some(duration); 
    }
    else if (path.extension().is_some_and(|ext| ext == "mp3")) {
        let duration = mp3_duration::from_path(&path).unwrap() as u64;
        return Some(duration)
    }
    else{
        let reader = WavReader::open(&path)?;
        let spec = reader.spec();
        let sample_rate = spec.sample_rate as u64;
        let num_samples = reader.duration(); // total samples (all channels)

        let duration_secs = num_samples as f64 / sample_rate as f64 / spec.channels as f64;
        Some(duration)
    }
    

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



pub fn wav_duration(){

}
pub fn time_pars(mut sec: u64) -> Vec<String> {
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