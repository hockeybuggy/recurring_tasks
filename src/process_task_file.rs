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

[tasks]

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
}
