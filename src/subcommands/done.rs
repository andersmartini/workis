use std::fs::remove_file;
use std::io::{BufRead, BufReader};

use chrono::{NaiveDateTime, Utc};

use crate::{Cli, utils};

pub fn store_timer_result(args: &Cli) {
    let mut tmp_file = utils::read_file(&utils::get_tmp_file_path()).unwrap();
    let reader = BufReader::new(&mut tmp_file);
    let last_line = reader
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

    let newline = format!("{} {} {}",
                          now.date(),
                          hours,
                          minutes);

    println!("adding work to log: {}", newline);
    utils::append_to_file(&utils::get_current_logfile_path(), &newline);

    remove_file(utils::get_tmp_file_path())
        .expect("Failed to remove tmp file, this probably isn't a big deal though");
}
