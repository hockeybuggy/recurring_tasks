use std::fs::File;
use std::io::prelude::*;
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
    println!("\nUsing input file: {}\n", source_path.to_str().unwrap());

    let (subject, body) = run_from_task_file(source_path);

    let mut subject_file = File::create("subject.txt").unwrap();
    println!("Using output subject file: subject.txt");
    subject_file.write_all(&subject.as_bytes()).unwrap();

    let mut body_file = File::create("body.txt").unwrap();
    println!("Using output body file: body.txt");
    body_file.write_all(&body.as_bytes()).unwrap();
}
