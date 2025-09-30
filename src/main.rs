mod config;
mod models;
mod utils;
mod parsing;

use crate::config::Config;
use models::{get_album, get_multi_album};
use parsing::{parse_album};

use std::env;
use std::io::{Error, Write};
use std::process;
use std::fs::File;


fn main() {
    let configuration = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments {err}");
        process::exit(1);
    });
    if configuration.is_discography() {
        let albums = get_multi_album(configuration.file_path);
    }
    else{
        let album = get_album(configuration.file_path).unwrap();
        let result = parse_album(album).unwrap();
        println!("{result}");
    }

}
