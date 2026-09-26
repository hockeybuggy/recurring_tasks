extern crate chrono;
extern crate cron;
use chrono::{DateTime, NaiveDate, TimeZone, Utc};

mod format_output;
mod process_task_file;
mod tasks;

use crate::tasks::Task;

pub fn run_from_task_file(source_path: &std::path::Path) -> (String, String) {
    let (timezone, tasks) = crate::process_task_file::parse_toml_file(source_path).unwrap();

    let now: DateTime<Utc> = Utc::now();
    tasks_for_day(&tasks, timezone, now.with_timezone(&timezone).date_naive())
}

fn tasks_for_day(tasks: &[Task], timezone: chrono_tz::Tz, date: NaiveDate) -> (String, String) {
    let local_datetime = timezone
        .from_local_datetime(&date.and_hms_opt(0, 0, 0).unwrap())
        .single()
        .unwrap();
    let next_midnight = timezone
        .from_local_datetime(&date.succ_opt().unwrap().and_hms_opt(0, 0, 0).unwrap())
        .single()
        .unwrap();
    let day = next_midnight - local_datetime;
    let upcoming = crate::tasks::get_tasks_occurring_within_duration(tasks, &local_datetime, &day);

    let message = crate::format_output::format_message(&upcoming, &local_datetime, &day);
    let subject = crate::format_output::format_subject(&upcoming, &local_datetime, &day);
    (subject, message)
}

#[cfg(test)]
mod recurring_task_tests {
    use super::tasks_for_day;
    use crate::tasks::Task;
    use chrono::NaiveDate;

    #[test]
    fn test_spring_forward_excludes_next_day_task() {
        let tasks = vec![Task {
            cron_expression: "0 30 0 9 3 * *".to_owned(),
            task_name: "Next day task".to_owned(),
        }];

        let (_, message) = tasks_for_day(
            &tasks,
            chrono_tz::America::Toronto,
            NaiveDate::from_ymd_opt(2020, 3, 8).unwrap(),
        );

        assert!(message.contains("There are no upcoming tasks."));
    }

    #[test]
    fn test_fall_back_includes_late_task() {
        let tasks = vec![Task {
            cron_expression: "0 30 23 1 11 * *".to_owned(),
            task_name: "Late task".to_owned(),
        }];

        let (_, message) = tasks_for_day(
            &tasks,
            chrono_tz::America::Toronto,
            NaiveDate::from_ymd_opt(2020, 11, 1).unwrap(),
        );

        assert!(message.contains(" - Late task\n"));
    }
}
