use chrono::{DateTime, Utc};
use crate::start::static_values::TIME_FORMAT;

#[path= "../utils.rs"]
mod utils;
#[path= "../static_values.rs"]
mod static_values;

pub fn store_now() {
    let now: DateTime<Utc> = Utc::now().into();

    let newline = format!("{}", now.format(TIME_FORMAT));

    utils::append_to_file(&utils::get_tmp_file_path(), &newline);
}
