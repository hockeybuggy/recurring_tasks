use std::fs;
use std::path::Path;

use chrono_tz::Tz;
use toml::Value as Toml;

use crate::Task;

pub fn parse_toml_file(source_path: &Path) -> Result<(chrono_tz::Tz, Vec<Task>), String> {
    let contents = fs::read_to_string(source_path).expect("Unable to read the source file");
    let parsed: Toml = match contents.parse() {
        Ok(toml) => toml,
        Err(error) => return Err(format!("Could not parse toml: {}", error)),
    };

    let maybe_timezone = parsed.get("timezone");
    if maybe_timezone.is_none() {
        return Err("Missing required `timezone` field.".to_owned());
    }
    let maybe_timezone_string = maybe_timezone.unwrap().as_str();
    if maybe_timezone_string.is_none() {
        return Err(
            "`timezone` field should be a string containing a valid timezone name.".to_owned(),
        );
    }
    let local_timezone: Tz = maybe_timezone_string.unwrap().parse()?;

    let mut tasks: Vec<Task> = vec![];

    let tables: Vec<(&String, &Toml)> = parsed
        .as_table()
        .unwrap()
        .iter()
        .filter(|key_value_pair| key_value_pair.1.is_table())
        .collect();
    if tables.len() == 0 {
        return Err("Could not find tasks.".to_owned());
    }

    for tasks_table in tables {
        let top_level_table_name = tasks_table.0;
        for (task_table_name, task_table) in tasks_table.1.as_table().unwrap() {
            let maybe_task_name = task_table.get("name");
            if maybe_task_name.is_none() {
                return Err(format!(
                    "Task `{}.{}` is missing a `name`.",
                    top_level_table_name, task_table_name
                ));
            }
            let maybe_task_name_str = maybe_task_name.unwrap().as_str();
            if maybe_task_name_str.is_none() {
                return Err(format!(
                    "Task `{}.{}` has a `name` field that is not a string.",
                    top_level_table_name, task_table_name
                ));
            }
            let task_name = maybe_task_name_str.unwrap().to_string();

            let maybe_cron_expression = task_table.get("cron_expression");
            if maybe_cron_expression.is_none() {
                return Err(format!(
                    "Task `{}.{}` is missing a `cron_expression`.",
                    top_level_table_name, task_table_name
                ));
            }
            let maybe_cron_expression_str = maybe_cron_expression.unwrap().as_str();
            if maybe_cron_expression_str.is_none() {
                return Err(format!(
                    "Task `{}.{}` has a `cron_expression` field that is not a string.",
                    top_level_table_name, task_table_name
                ));
            }
            let cron_expression = maybe_cron_expression_str.unwrap().to_string();
            // TODO maybe validate that this is a valid cron string

            tasks.push(Task {
                task_name,
                cron_expression,
            });
        }
    }
    return Ok((local_timezone, tasks));
}

#[cfg(test)]
mod process_task_file_tests {
    use std::env;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::PathBuf;

    use crate::process_task_file::parse_toml_file;
    use crate::tasks::Task;

    fn write_to_tempfile(file_name: &str, file_contents: &str) -> PathBuf {
        let dir = env::temp_dir();
        let source_path = dir.join(file_name);
        let mut temp_file = File::create(&source_path).unwrap();
        temp_file.write_all(&file_contents.as_bytes()).unwrap();

        return source_path;
    }

    #[test]
    fn test_parse_toml_file_valid_file() {
        let source_path = write_to_tempfile(
            "valid_simple_example.toml",
            "
timezone = 'America/Toronto'

[tasks.water]
name = 'Drink water (min 1 litre)'
cron_expression = '0 0 * * * * *'

[tasks.teeth]
name = 'Brush teeth'
cron_expression = '0 0 * * * * *'
",
        );

        let (local_timezone, tasks) = parse_toml_file(&source_path).unwrap();

        let expected_timezone = chrono_tz::America::Toronto;
        assert_eq!(local_timezone, expected_timezone);
        let expected_tasks: Vec<Task> = vec![
            Task {
                cron_expression: "0 0 * * * * *".to_owned(),
                task_name: "Brush teeth".to_owned(),
            },
            Task {
                cron_expression: "0 0 * * * * *".to_owned(),
                task_name: "Drink water (min 1 litre)".to_owned(),
            },
        ];
        assert_eq!(tasks, expected_tasks);
    }

    #[test]
    fn test_parse_toml_file_multiple_task_tables() {
        let source_path = write_to_tempfile(
            "multiple_task_tables.toml",
            "
timezone = 'America/Toronto'

[tasks.water]
name = 'Drink water (min 1 litre)'
cron_expression = '0 0 * * * * *'

[birthdays.neo]
name = 'Thomas Anderson Birthday'
cron_expression = '0 0 * 3 11 * *'
",
        );

        let (local_timezone, tasks) = parse_toml_file(&source_path).unwrap();

        let expected_timezone = chrono_tz::America::Toronto;
        assert_eq!(local_timezone, expected_timezone);
        let expected_tasks: Vec<Task> = vec![
            Task {
                cron_expression: "0 0 * 3 11 * *".to_owned(),
                task_name: "Thomas Anderson Birthday".to_owned(),
            },
            Task {
                cron_expression: "0 0 * * * * *".to_owned(),
                task_name: "Drink water (min 1 litre)".to_owned(),
            },
        ];
        assert_eq!(tasks, expected_tasks);
    }

    #[test]
    fn test_parse_toml_file_missing_timezone() {
        let source_path = write_to_tempfile(
            "missing_timezone.toml",
            "
# This task file is mising the required `timezone` field.

[tasks]

[tasks.water]
name = 'Drink water (min 1 litre)'
cron_expression = '0 0 * * * * *'
",
        );

        let err = parse_toml_file(&source_path).unwrap_err();

        let expected_err = "Missing required `timezone` field.";
        assert_eq!(err, expected_err);
    }

    #[test]
    fn test_parse_toml_file_timezone_of_wrong_type() {
        let source_path = write_to_tempfile(
            "timezone_of_wrong_type.toml",
            "
# This task file has `timezone` field of the wrong type. It should be a string.
timezone = 1

[tasks]

[tasks.water]
name = 'Drink water (min 1 litre)'
cron_expression = '0 0 * * * * *'
",
        );

        let err = parse_toml_file(&source_path).unwrap_err();

        let expected_err = "`timezone` field should be a string containing a valid timezone name.";
        assert_eq!(err, expected_err);
    }

    #[test]
    fn test_parse_toml_file_timezone_that_does_not_exist() {
        let source_path = write_to_tempfile(
            "invalid_timezone.toml",
            "
# This task file has `timezone` field that doesn't exist in the timezone db
timezone = 'Pangaea/FutureToronto'

[tasks]

[tasks.water]
name = 'Drink water (min 1 litre)'
cron_expression = '0 0 * * * * *'
",
        );

        let err = parse_toml_file(&source_path).unwrap_err();

        let expected_err = "'Pangaea/FutureToronto\' is not a valid timezone";
        assert_eq!(err, expected_err);
    }

    #[test]
    fn test_parse_toml_file_missing_tasks_table() {
        let source_path = write_to_tempfile(
            "missing_tasks_table.toml",
            "
timezone = 'America/Toronto'

# This task file doesn't have a Table containing tasks
",
        );

        let err = parse_toml_file(&source_path).unwrap_err();

        let expected_err = "Could not find tasks.";
        assert_eq!(err, expected_err);
    }

    #[test]
    fn test_parse_toml_file_task_with_missing_name() {
        let source_path = write_to_tempfile(
            "task_with_missing_name.toml",
            "
timezone = 'America/Toronto'

# This task file has a task without a name
[tasks.some_task]
cron_expression = '* * * * * * *'
",
        );

        let err = parse_toml_file(&source_path).unwrap_err();

        let expected_err = "Task `tasks.some_task` is missing a `name`.";
        assert_eq!(err, expected_err);
    }

    #[test]
    fn test_parse_toml_file_task_with_a_non_string_name() {
        let source_path = write_to_tempfile(
            "task_with_non_string_name.toml",
            "
timezone = 'America/Toronto'

# This task file has a task a non string name
[tasks.some_task]
name = 1
cron_expression = '* * * * * * *'
",
        );

        let err = parse_toml_file(&source_path).unwrap_err();

        let expected_err = "Task `tasks.some_task` has a `name` field that is not a string.";
        assert_eq!(err, expected_err);
    }

    #[test]
    fn test_parse_toml_file_task_with_missing_cron_expression() {
        let source_path = write_to_tempfile(
            "task_with_missing_cron_expression.toml",
            "
timezone = 'America/Toronto'

# This task file has a task without a cron_expression
[tasks.some_task]
name = 'Some task'
",
        );

        let err = parse_toml_file(&source_path).unwrap_err();

        let expected_err = "Task `tasks.some_task` is missing a `cron_expression`.";
        assert_eq!(err, expected_err);
    }

    #[test]
    fn test_parse_toml_file_task_with_a_non_string_cron_expression() {
        let source_path = write_to_tempfile(
            "task_with_non_string_cron_expression.toml",
            "
timezone = 'America/Toronto'

# This task file has a task cron_expression that is not a string
[tasks.some_task]
name = 'Some task'
cron_expression = 1
",
        );

        let err = parse_toml_file(&source_path).unwrap_err();

        let expected_err =
            "Task `tasks.some_task` has a `cron_expression` field that is not a string.";
        assert_eq!(err, expected_err);
    }
}
