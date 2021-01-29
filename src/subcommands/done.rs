use crate::{Cli, utils};
use std::io::{BufReader, BufRead};
use chrono::{NaiveDateTime, Utc};
use std::fs::remove_file;
use crate::done::static_values::TIME_FORMAT;

#[path= "../static_values.rs"]
mod static_values;

pub fn store_timer_result(args: Cli) {
    let mut tmp_file = utils::read_file(&utils::get_tmp_file_path()).unwrap();
    let reader = BufReader::new(&mut tmp_file);
    let mut last_line = reader
        .lines()
        .map(|l| l.expect("couldnt read a line"))
        .last()
        .expect("file was empty");

    let startTime = utils::parse_time_string(&last_line);
    let now: NaiveDateTime = Utc::now().naive_utc();
    let diff = now.signed_duration_since(startTime);

    let actual_minutes = diff.num_minutes() - args.lunch;
    let hours = actual_minutes / 60;
    let minutes = actual_minutes % 60;

    let newline = format!("{} : {}:{}",
                          now.format(TIME_FORMAT),
                          hours,
                          minutes);

    println!("adding work to log: {}", newline);
    utils::append_to_file(&utils::get_current_logfile_path(), &newline);

    remove_file(utils::get_tmp_file_path());
}
