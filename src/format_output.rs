use crate::Task;

pub fn format_upcoming_tasks_into_message(upcoming: &Vec<&Task>) -> String {
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
