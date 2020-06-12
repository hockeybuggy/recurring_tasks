use std::process;

use chrono::{DateTime, Utc};
use recurring_tasks::{display_upcoming_tasks, get_tasks_occurring_in_the_next_hour, Task};

fn main() {
    // TODO I hope to make this an external file that can be read in.
    let tasks: Vec<Task> = vec![
        Task {
            cron_expression: "0 0 16 * * 1-5 *".to_owned(),
            task_name: "Drink water (min 1 litre)".to_owned(),
        },
        Task {
            cron_expression: "0 0 16 * * 1-5 *".to_owned(),
            task_name: "Brush teeth".to_owned(),
        },
        Task {
            cron_expression: "0 0 16 * * 1-5 *".to_owned(),
            task_name: "Light therapy".to_owned(),
        },
        Task {
            cron_expression: "0 0 16 * * 1-5 *".to_owned(),
            task_name: "Garbage, Green bin".to_owned(),
        },
        Task {
            cron_expression: "0 0 16 * * 1-5 *".to_owned(),
            task_name: "Recycling, Green bin".to_owned(),
        },
        Task {
            cron_expression: "0 0 16 * * 1-5 *".to_owned(),
            task_name: "Pay rent".to_owned(),
        },
        Task {
            cron_expression: "0 0 16 * * 1-5 *".to_owned(),
            task_name: "Pay joint account contribution".to_owned(),
        },
        Task {
            cron_expression: "0 0 16 * * 1-5 *".to_owned(),
            task_name: "Pay credit card".to_owned(),
        },
        Task {
            cron_expression: "0 0 16 * * 1-5 *".to_owned(),
            task_name: "Pay taxes".to_owned(),
        },
        Task {
            cron_expression: "0 0 16 * * 1-5 *".to_owned(),
            task_name: "Rotate passwords".to_owned(),
        },
    ];

    let now: DateTime<Utc> = Utc::now();
    let upcoming = get_tasks_occurring_in_the_next_hour(&tasks, now);

    if upcoming.len() == 0 {
        println!("There are no upcoming tasks.");
        process::exit(1);
    }

    display_upcoming_tasks(&upcoming);
}
