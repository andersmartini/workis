use std::ffi::OsString;
use std::fs::{self, DirEntry};
use std::io;
use std::io::{BufRead, Error};
use std::panic::resume_unwind;
use std::path::{Path, PathBuf};

use crate::Cli;

#[path = "../utils.rs"]
mod utils;
#[path = "../static_values.rs"]
mod static_values;

pub fn sum_monthly_work(args: &Cli) {
    let result = list_files_in_path(&utils::get_storage_catalog_dir())
        .unwrap();

    let chosen_file = ask_user_to_pick_file(result);



}

fn ask_user_to_pick_file(available_files: Vec<DirEntry>) -> DirEntry {
    println!("Which month would you like to sum?");
    print_file_names(&available_files);
    let input = read_line_from_stdin();

    let chosen = available_files.into_iter()
        .find(|dir_entry| dir_entry.file_name().into_string().unwrap() == input)
        .expect("didnt match any files");

    return chosen;
}


fn read_line_from_stdin() -> String {
    io::stdin()
        .lock()
        .lines()
        .next()
        .expect("no new input")
        .expect("failed to read input")
}

fn print_file_names(files: &Vec<DirEntry>) {
    for dir in files {
        let filename: String = dir.file_name().into_string().unwrap();
        println!("{}", filename)
    }
}


fn list_files_in_path(path: &PathBuf) -> Result<Vec<DirEntry>, Error> {
    let mut files = fs::read_dir(path)?
        .collect::<Result<Vec<_>, io::Error>>();
    files
}