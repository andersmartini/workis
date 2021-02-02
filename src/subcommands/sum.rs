use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::{self, DirEntry};
use std::io;
use std::io::{BufRead, BufReader, Error};
use std::panic::resume_unwind;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::Cli;

#[path = "../utils.rs"]
mod utils;
#[path = "../static_values.rs"]
mod static_values;

pub fn sum_monthly_work(args: &Cli) {
    let result = list_files_in_path(&utils::get_storage_catalog_dir())
        .unwrap();

    let chosen_file_dir_entry = ask_user_to_pick_file(result);

    let mut file = utils::read_file(&chosen_file_dir_entry.path())
        .expect("failed to read the chosen file");

    let reader = BufReader::new(&mut file);
    let log_entries: Vec<LogEntry> = reader
        .lines()
        .map(|l| l.expect("couldnt read a line"))
        .map(|s| parse_log_entry(&s))
        .collect();

    let mut entry_map: HashMap<String, LogEntry> = HashMap::new();
    let mut monthly_total_minutes = 0;

    for entry in &log_entries {
        match entry_map.get(&entry.date) {
            Some(log_entry) => entry_map.insert(entry.date.clone(), sum_time_of_entries(log_entry, &entry)),
            None => entry_map.insert(entry.date.clone(), entry.clone())
        };
    }

    for (key, value) in entry_map {
        println!("you worked {} hours and {} minutes on {}", value.hrs, value.minutes, value.date);
        monthly_total_minutes += (value.hrs*60) + value.minutes
    }
    let monthly_hrs = monthly_total_minutes/60;
    let monthly_spare_minutes = monthly_total_minutes %60;

    println!("you worked a total of {} hours and {} minutes during {}", monthly_hrs, monthly_spare_minutes, chosen_file_dir_entry.file_name().into_string().unwrap()  )
}


fn sum_time_of_entries(a: &LogEntry, b: &LogEntry) -> LogEntry {
    let hrs_sum = a.hrs + b.hrs;
    let total_minutes = (hrs_sum * 60) + a.minutes + b.minutes;

    return LogEntry { date: a.date.clone(), hrs: total_minutes / 60, minutes: total_minutes % 60 };
}

fn parse_log_entry(entry: &String) -> LogEntry {
    let mut split: Vec<String> = entry.split(" ").map(|s| s.to_string()).collect();
    let date = split[0].clone();
    let hrs = split[1].parse::<i64>().expect("couldnt parse hours from log");
    let minutes = split[2].parse::<i64>().expect("couldnt parse minutes from log");

    LogEntry { date, hrs, minutes }
}

#[derive(Clone)]
struct LogEntry {
    date: String,
    hrs: i64,
    minutes: i64,
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