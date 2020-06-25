extern crate chrono;
extern crate cron;
use chrono::{DateTime, Timelike, Utc};

mod format_output;
mod process_task_file;
mod tasks;

use crate::tasks::Task;

pub fn run_from_task_file(source_path: &std::path::Path) -> (String, String) {
    let (timezone, tasks) = crate::process_task_file::parse_toml_file(&source_path).unwrap();

    let now: DateTime<Utc> = Utc::now();
    let local_datetime = now
        .with_timezone(&timezone)
        .with_hour(0)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap()
        .with_nanosecond(0)
        .unwrap();

    let day = chrono::Duration::days(1) - chrono::Duration::seconds(1);
    let upcoming = crate::tasks::get_tasks_occurring_within_duration(&tasks, &local_datetime, &day);

    let message = crate::format_output::format_message(&upcoming, &local_datetime, &day);
    let subject = crate::format_output::format_subject(&upcoming, &local_datetime, &day);
    return (subject, message);
}

#[cfg(test)]
mod recurring_task_tests {}
