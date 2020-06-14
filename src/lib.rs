extern crate chrono;
extern crate cron;

use chrono_tz::Tz;
use cron::Schedule;
use std::str::FromStr;

#[derive(Debug)]
pub struct Task {
    pub cron_expression: String,
    pub task_name: String,
}

pub fn get_tasks_occurring_in_the_next_hour(
    tasks: &Vec<Task>,
    now: chrono::DateTime<Tz>,
) -> Vec<&Task> {
    let hour = chrono::Duration::hours(1);

    let mut upcoming: Vec<&Task> = Vec::new();
    for task in tasks {
        let schedule = Schedule::from_str(&task.cron_expression).unwrap();
        for next_occurance in schedule.after(&now).take(1) {
            if next_occurance > now && next_occurance <= now + hour {
                upcoming.push(task);
            }
        }
    }
    return upcoming;
}

pub fn display_upcoming_tasks(upcoming: &Vec<&Task>) {
    println!("These are the upcoming tasks:");
    for task in upcoming {
        println!("\t{}", task.task_name);
    }
}

#[cfg(test)]
mod recurring_task_tests {
    use crate::{get_tasks_occurring_in_the_next_hour, Task};
    use chrono::TimeZone;

    #[test]
    fn test_includes_task_that_occurs_in_the_next_hour() {
        let every_day_at_ten = "0 0 10 * * * *".to_owned();
        let tasks = vec![Task {
            cron_expression: every_day_at_ten,
            task_name: "Get new light jacket".to_owned(),
        }];
        let the_perfect_date_morning = chrono_tz::America::Toronto
            .ymd(2020, 4, 25)
            .and_hms(9, 10, 11);
        let upcoming = get_tasks_occurring_in_the_next_hour(&tasks, the_perfect_date_morning);
        assert_eq!(upcoming.len(), 1);
    }

    #[test]
    fn test_includes_task_that_does_not_occur_in_the_next_hour() {
        let every_day_at_ten = "0 0 16 * * 1-5 *".to_owned();
        let tasks = vec![Task {
            cron_expression: every_day_at_ten,
            task_name: "Get new light jacket".to_owned(),
        }];
        let the_perfect_date_afternoon = chrono_tz::America::Toronto
            .ymd(2020, 4, 25)
            .and_hms(16, 21, 00);
        let upcoming = get_tasks_occurring_in_the_next_hour(&tasks, the_perfect_date_afternoon);
        assert_eq!(upcoming.len(), 0);
    }
}
