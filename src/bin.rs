use std::path::Path;

extern crate clap;
use clap::{App, Arg};

use recurring_tasks::run_from_task_file;

const ABOUT_BLERB: &str = "
This project outputs a task list given a schedule.
";

fn main() {
    let matches = App::new("Recurring tasks")
        .version("0.1.0")
        .author("Douglas Anderson <hockeybuggy@gmail.com>")
        .about(ABOUT_BLERB)
        .arg(
            Arg::with_name("tasks")
                .help("Sets the input task file to use")
                .takes_value(true)
                .short("t")
                .long("tasks")
                .required(true),
        )
        .get_matches();

    let source_path = Path::new(matches.value_of("tasks").unwrap());

    run_from_task_file(source_path);
}
