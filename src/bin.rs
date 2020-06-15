use std::path::Path;

extern crate clap;
use chrono::{DateTime, Utc};
use clap::{App, Arg};

use recurring_tasks::{
    display_upcoming_tasks, get_tasks_occurring_within_duration, parse_toml_file,
};

const ABOUT_BLERB: &str = "
This project outputs a task list given a schedule.
";

fn main() {
    let matches = App::new("ValueType Codegen")
        .version("0.1.0")
        .author("Douglas Anderson <hockeybuggy@gmail.com>")
        .about(ABOUT_BLERB)
        .arg(
            Arg::with_name("source")
                .help("Sets the input file to use")
                .takes_value(true)
                .short("s")
                .long("source")
                .required(true),
        )
        .get_matches();

    let source_path = Path::new(matches.value_of("source").unwrap());
    let (timezone, tasks) = parse_toml_file(source_path).unwrap();

    let now: DateTime<Utc> = Utc::now();
    let local_datetime = now.with_timezone(&timezone);

    let day = chrono::Duration::days(1);
    let upcoming = get_tasks_occurring_within_duration(&tasks, local_datetime, day);

    println!(
        "Tasks between:\n\t- {:?}\n\t- {:?}",
        local_datetime,
        local_datetime + day
    );
    println!("");
    display_upcoming_tasks(&upcoming);
}
