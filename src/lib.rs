use std::fs;
use std::path::Path;
use std::str::FromStr;

extern crate chrono;
extern crate cron;
use chrono_tz::Tz;
use cron::Schedule;

use toml::Value as Toml;

#[derive(Debug)]
pub struct Task {
    pub cron_expression: String,
    pub task_name: String,
}

pub fn get_tasks_occurring_within_duration(
    tasks: &Vec<Task>,
    now: chrono::DateTime<Tz>,
    duration: chrono::Duration,
) -> Vec<&Task> {
    let mut upcoming: Vec<&Task> = Vec::new();
    for task in tasks {
        let schedule = Schedule::from_str(&task.cron_expression).unwrap();
        for next_occurance in schedule.after(&now).take(1) {
            if next_occurance > now && next_occurance <= now + duration {
                upcoming.push(task);
            }
        }
    }
    return upcoming;
}

pub fn display_upcoming_tasks(upcoming: &Vec<&Task>) {
    if upcoming.len() == 0 {
        println!("There are no upcoming tasks.");
    } else {
        println!("These are the upcoming tasks:");
        for task in upcoming {
            println!("    - {}", task.task_name);
        }
    }
}

pub fn parse_toml_file(source_path: &Path) -> Result<(chrono_tz::Tz, Vec<Task>), String> {
    println!("Using input file: {}", source_path.to_str().unwrap());
    let contents = fs::read_to_string(source_path).expect("Unable to read the source file");
    let parsed: Toml = match contents.parse() {
        Ok(toml) => toml,
        Err(error) => return Err(format!("Could not parse toml: {}", error)),
    };

    // TODO This parsing could be better.
    let local_timezone: Tz = parsed
        .get("timezone")
        .unwrap()
        .as_str()
        .unwrap()
        .parse()
        .unwrap();

    let mut tasks: Vec<Task> = vec![];

    let raw_tasks = parsed.get("tasks").unwrap();
    for (_task_name, task_table) in raw_tasks.as_table().unwrap() {
        tasks.push(Task {
            task_name: task_table
                .get("name")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
            cron_expression: task_table
                .get("cron_expression")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
        });
    }
    return Ok((local_timezone, tasks));
}

#[cfg(test)]
mod recurring_task_tests {
    use crate::{get_tasks_occurring_within_duration, Task};
    use chrono::TimeZone;

    #[test]
    fn test_includes_task_that_occurs_in_the_next_hour() {
        let hour = chrono::Duration::hours(1);
        let every_day_at_ten = "0 0 10 * * * *".to_owned();
        let tasks = vec![Task {
            cron_expression: every_day_at_ten,
            task_name: "Get new light jacket".to_owned(),
        }];
        let the_perfect_date_morning = chrono_tz::America::Toronto
            .ymd(2020, 4, 25)
            .and_hms(9, 10, 11);
        let upcoming = get_tasks_occurring_within_duration(&tasks, the_perfect_date_morning, hour);
        assert_eq!(upcoming.len(), 1);
    }

    #[test]
    fn test_includes_task_that_does_not_occur_in_the_next_hour() {
        let hour = chrono::Duration::hours(1);
        let every_day_at_ten = "0 0 16 * * 1-5 *".to_owned();
        let tasks = vec![Task {
            cron_expression: every_day_at_ten,
            task_name: "Get new light jacket".to_owned(),
        }];
        let the_perfect_date_afternoon = chrono_tz::America::Toronto
            .ymd(2020, 4, 25)
            .and_hms(16, 21, 00);
        let upcoming =
            get_tasks_occurring_within_duration(&tasks, the_perfect_date_afternoon, hour);
        assert_eq!(upcoming.len(), 0);
    }
}
