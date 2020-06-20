use crate::Task;

fn format_time_span(
    start_datetime: &chrono::DateTime<chrono_tz::Tz>,
    duration: &chrono::Duration,
) -> String {
    let time_message = format!(
        "Tasks between:\n\n - {}\n - {}\n",
        start_datetime.format("%F %T %:z"),
        (*start_datetime + *duration).format("%F %T %:z")
    );
    return time_message;
}

fn format_upcoming_tasks_into_message(upcoming: &Vec<&Task>) -> String {
    let mut msg = String::from("");
    if upcoming.len() == 0 {
        msg.push_str(&format!("There are no upcoming tasks.\n"));
    } else {
        msg.push_str(&format!("These are the upcoming tasks:\n\n"));

        for task in upcoming {
            msg.push_str(&format!(" - {}\n", task.task_name));
        }
    }
    return msg;
}

/// Create a message string given a list of empty tasks and information about the time span is for.
/// This will be used as the text body version of the email. The message should be markdown for
/// generating the html version of the email.
pub fn format_message(
    upcoming: &Vec<&Task>,
    start_datetime: &chrono::DateTime<chrono_tz::Tz>,
    duration: &chrono::Duration,
) -> String {
    let message = format!(
        "{}\n{}",
        crate::format_output::format_time_span(start_datetime, duration),
        crate::format_output::format_upcoming_tasks_into_message(upcoming),
    );
    return message;
}

/// Create a message string given a list of empty tasks and information about the time span is for.
/// This message will be used as the subject line for the email.
pub fn format_subject(
    _upcoming: &Vec<&Task>,
    start_datetime: &chrono::DateTime<chrono_tz::Tz>,
    _duration: &chrono::Duration,
) -> String {
    let message = format!("Recurring tasks for {}", start_datetime.format("%F"));
    return message;
}

#[cfg(test)]
mod format_output_tests {
    use crate::format_output::{
        format_message, format_subject, format_time_span, format_upcoming_tasks_into_message,
    };
    use crate::tasks::Task;
    use chrono::TimeZone;

    #[test]
    fn test_format_upcoming_tasks_into_message_empty_tasks() {
        let message = format_upcoming_tasks_into_message(&vec![]);
        assert_eq!(message, "There are no upcoming tasks.\n");
    }

    #[test]
    fn test_format_upcoming_tasks_into_message_some_task() {
        let task1 = Task {
            cron_expression: "0 0 10 * * * *".to_owned(),
            task_name: "Get new light jacket".to_owned(),
        };
        let task2 = Task {
            cron_expression: "0 0 10 * * * *".to_owned(),
            task_name: "Get tickets for date night".to_owned(),
        };

        let message = format_upcoming_tasks_into_message(&vec![&task1, &task2]);

        assert_eq!(
            message,
            "These are the upcoming tasks:\n\n - Get new light jacket\n - Get tickets for date night\n"
        );
    }

    #[test]
    fn test_format_time_span() {
        let the_perfect_date_afternoon = chrono_tz::America::Toronto
            .ymd(2020, 4, 25)
            .and_hms(16, 21, 00);
        let hour = chrono::Duration::hours(1);

        let message = format_time_span(&the_perfect_date_afternoon, &hour);

        assert_eq!(
            message,
            "Tasks between:\n\n - 2020-04-25 16:21:00 -04:00\n - 2020-04-25 17:21:00 -04:00\n"
        );
    }

    #[test]
    fn test_format_message() {
        let the_perfect_date_afternoon = chrono_tz::America::Toronto
            .ymd(2020, 4, 25)
            .and_hms(16, 21, 00);

        let hour = chrono::Duration::hours(1);
        let tasks = &vec![];

        let message = format_message(&tasks, &the_perfect_date_afternoon, &hour);

        let expected_message = format!(
            "{}\n{}",
            format_time_span(&the_perfect_date_afternoon, &hour),
            format_upcoming_tasks_into_message(&tasks),
        );
        assert_eq!(message, expected_message);
    }

    #[test]
    fn test_format_subject() {
        let the_perfect_date_afternoon = chrono_tz::America::Toronto
            .ymd(2020, 4, 25)
            .and_hms(16, 21, 00);

        let hour = chrono::Duration::hours(1);
        let tasks = &vec![];

        let message = format_subject(&tasks, &the_perfect_date_afternoon, &hour);

        let expected_message = format!(
            "Recurring tasks for {}",
            the_perfect_date_afternoon.format("%F")
        );
        assert_eq!(message, expected_message);
    }
}
