use std::path::Path;

extern crate clap;
use chrono::{DateTime, Utc};
use clap::{App, Arg};

use recurring_tasks::{
    display_upcoming_tasks, get_tasks_occurring_in_the_next_hour, parse_toml_file,
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
    // TODO I hope to make this an external file that can be read in.
    let now: DateTime<Utc> = Utc::now();

    let (timezone, tasks) = parse_toml_file(source_path).unwrap();

    let local_datetime = now.with_timezone(&timezone);

    let upcoming = get_tasks_occurring_in_the_next_hour(&tasks, local_datetime);

    println!("It is now {:?} in UTC", now);
    println!("      and {:?} in {:?}", local_datetime, timezone);
    println!("");
    display_upcoming_tasks(&upcoming);
}
