use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{Write};
use std::path::PathBuf;

use chrono::{Datelike};
use chrono::NaiveDateTime;
use chrono::offset::Utc;

static time_format: &'static str = "%Y-%m-%d %H:%M:%S";

pub fn read_file(path: &PathBuf) -> std::io::Result<File> {
    return OpenOptions::new()
        .read(true)
        .open(path);
}

pub fn append_to_file(path: &PathBuf, line: &String) {
    create_dir_all(get_storage_catalog_dir());

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", line) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

pub fn get_storage_catalog_dir() -> PathBuf {
    return dirs::home_dir().unwrap().join("./.rusttime");
}

pub fn get_tmp_file_path() -> PathBuf {
    return get_storage_catalog_dir().join("./tmp.txt");
}

pub fn get_current_logfile_path() -> PathBuf {
    let now = Utc::now().naive_utc();
    let filename = format!("{}-{}.txt", now.year(), now.month());
    let subpath = format!("./{}", filename);
    return get_storage_catalog_dir().join(subpath);
}

pub fn parse_time_string(timestring: &String) -> NaiveDateTime {
    return NaiveDateTime::parse_from_str(timestring, time_format).unwrap();
}
