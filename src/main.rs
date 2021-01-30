use chrono::offset::Utc;
use structopt::StructOpt;

mod utils;
#[path = "subcommands/start.rs"]
mod start;
#[path = "subcommands/done.rs"]
mod done;

fn main() {
    let args = Cli::from_args();

    if args.subcommand == "add" {
        write_time(&args)
    }
    if args.subcommand == "start" {
        start::store_now()
    }
    if args.subcommand == "done" {
        done::store_timer_result(args)
    }
}

fn write_time(args: &Cli) {
    let day = Utc::now().date();
    let hrs = args.hrs.as_ref().expect("Hrs argument required for this operation");

    let newline = format!("day:{}, hrs:{}", day, hrs);

    utils::append_to_file(&utils::get_current_logfile_path(), &newline);
}



#[derive(StructOpt)]
pub struct Cli {
    subcommand: String,
    ///length of todays lunchbreak, in minutes
    #[structopt(short = "l", long = "lunch", default_value = "0")]
    lunch: i64,
    #[structopt(short = "h", long = "hours")]
    hrs: Option<f64>,
}

