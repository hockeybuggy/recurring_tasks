extern crate chrono;
extern crate cron;

use chrono::{DateTime, Utc};
use cron::Schedule;
use std::str::FromStr;

/*
  ┌───────────── sec (0 - 59)
  │ ┌───────────── minute (0 - 59)
  │ │ ┌───────────── hour (0 - 23)
  │ │ │ ┌───────────── day of the month (1 - 31)
  │ │ │ │ ┌───────────── month (1 - 12 or JAN-DEC)
  │ │ │ │ │ ┌───────────── day of the week (0 - 6 or SUN-SAT)
  │ │ │ │ │ │ ┌───────────── year
  │ │ │ │ │ │ │
  │ │ │ │ │ │ │
  * * * * * * *
*/

#[derive(Debug)]
struct Task {
    cron_expression: String,
    task_name: String,
}

fn get_tasks_occurring_in_the_next_hour(
    tasks: &Vec<Task>,
    now: chrono::DateTime<Utc>,
) -> Vec<&Task> {
    let mut upcoming: Vec<&Task> = Vec::new();
    for task in tasks {
        let schedule = Schedule::from_str(&task.cron_expression).unwrap();
        for datetime in schedule.after(&now).take(1) {
            upcoming.push(task);
        }
    }
    return upcoming;
}

fn main() {
    // let birthdays = vec![
    //     ("* * 7 6 *", "Wish myself a happy birthday".to_owned()
    // ];
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
    println!("Hello, world!");
    println!("{:?}", upcoming);
}
