use std::path::PathBuf;
use std::ffi::OsString;
use std::fs;

pub struct Config {
    pub file_path: PathBuf,
}
impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let mut path = match args.next() {
            Some(p) => p,
            None => return Err("didn't get a file path"),
        };

        let mut file_path = PathBuf::from(OsString::from(path));
        println!("{}", file_path.display());
        Ok(Config {
            file_path: file_path,
        })
    }
    pub fn is_discography(&self) -> bool {
        let entries = fs::read_dir(&self.file_path).unwrap();
        for mut entry in entries {
            let file_type = entry.unwrap().file_type().unwrap();
            if file_type.is_file() {
                return false;
            }
        }
        true
    }
}