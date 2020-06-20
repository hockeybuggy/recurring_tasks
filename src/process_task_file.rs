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
