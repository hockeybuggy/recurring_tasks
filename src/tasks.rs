use std::str::FromStr;

#[derive(Debug)]
pub struct Task {
    pub cron_expression: String,
    pub task_name: String,
}

/// Filters all tasks to just tasks ocurring within the period of `now` and `now + duration`.
pub fn get_tasks_occurring_within_duration<'a>(
    tasks: &'a Vec<Task>,
    now: &'a chrono::DateTime<chrono_tz::Tz>,
    duration: &'a chrono::Duration,
) -> Vec<&'a Task> {
    let mut upcoming: Vec<&Task> = Vec::new();
    for task in tasks {
        let schedule = cron::Schedule::from_str(&task.cron_expression).unwrap();
        for next_occurance in schedule.after(&now).take(1) {
            if next_occurance > *now && next_occurance <= *now + *duration {
                upcoming.push(task);
            }
        }
    }
    return upcoming;
}

#[cfg(test)]
mod tasks_tests {
    use crate::tasks::{get_tasks_occurring_within_duration, Task};
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

        let upcoming =
            get_tasks_occurring_within_duration(&tasks, &the_perfect_date_morning, &hour);

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
            get_tasks_occurring_within_duration(&tasks, &the_perfect_date_afternoon, &hour);

        assert_eq!(upcoming.len(), 0);
    }
}
