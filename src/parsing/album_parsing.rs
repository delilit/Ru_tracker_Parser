use crate::models::Album;
use crate::utils::time::*;

pub fn parse_album(album: Album) -> Result<String, &'static str>{

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