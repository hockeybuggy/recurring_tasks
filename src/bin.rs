use std::io::Write;

use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use recurring_tasks::{display_upcoming_tasks, get_tasks_occurring_in_the_next_hour, Task};

fn main() {
    // TODO I hope to make this an external file that can be read in.
    let local_timezone: Tz = "America/Toronto".parse().unwrap();
    let tasks: Vec<Task> = vec![
        Task {
            cron_expression: "0 0 3 * * * *".to_owned(),
            task_name: "Drink water (min 1 litre)".to_owned(),
        },
        Task {
            cron_expression: "0 0 * * * 1-5 *".to_owned(),
            task_name: "Brush teeth".to_owned(),
        },
        Task {
            cron_expression: "0 0 14 * * 1-5 *".to_owned(),
            task_name: "Light therapy".to_owned(),
        },
        Task {
            cron_expression: "0 0 16 * * 1-5 *".to_owned(),
            task_name: "Garbage, Green bin".to_owned(),
        },
        Task {
            cron_expression: "0 0 17 * * 1-5 *".to_owned(),
            task_name: "Recycling, Green bin".to_owned(),
        },
        Task {
            cron_expression: "0 0 18 * * 1-5 *".to_owned(),
            task_name: "Pay rent".to_owned(),
        },
        Task {
            cron_expression: "0 0 20 * * * *".to_owned(),
            task_name: "Pay joint account contribution".to_owned(),
        },
        Task {
            cron_expression: "0 0 21 * * * *".to_owned(),
            task_name: "Pay credit card".to_owned(),
        },
        Task {
            cron_expression: "0 0 23 * * * *".to_owned(),
            task_name: "Pay taxes".to_owned(),
        },
        Task {
            cron_expression: "0 0 22 * * * *".to_owned(),
            task_name: "Rotate passwords".to_owned(),
        },
    ];

    let now: DateTime<Utc> = Utc::now();
    let local_datetime = now.with_timezone(&local_timezone);
    println!("it is now {:?} in UTC", now);
    println!("      and {:?} in {:?}", local_datetime, local_timezone);

    let upcoming = get_tasks_occurring_in_the_next_hour(&tasks, local_datetime);

    if upcoming.len() == 0 {
        return std::io::stderr()
            .write_all(b"There are no upcoming tasks.")
            .unwrap();
    } else {
        display_upcoming_tasks(&upcoming);
    }
}
