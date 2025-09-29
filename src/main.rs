use std::env;
use std::io::{Error, Write};
use std::process;
use std::fs::File;
use ru_tracker_loss_less_parser::{get_multi_album, Config, get_album, parse_single_album};

fn main() {
    let configuration = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments {err}");
        process::exit(1);
    });
    if configuration.is_multi_album() {
        let albums = get_multi_album(configuration.file_path);
    }
    else{
        let album = get_album(configuration.file_path).unwrap();
        let result = parse_single_album(album).unwrap();
        println!("{result}");


    }

}
