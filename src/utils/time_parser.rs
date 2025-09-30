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