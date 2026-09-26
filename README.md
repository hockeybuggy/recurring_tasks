# Recurring tasks

![Build and Test](https://github.com/hockeybuggy/recurring_tasks/workflows/Build%20and%20Test/badge.svg)

This project generates a list of upcoming tasks from a TOML schedule. It does
not send reminders itself.

## How this works

This repository reads a task file, finds tasks scheduled for the current day,
and writes email-ready output files. A separate private repository runs it
periodically and handles notifications.


## Running tests

```bash
cargo test
```


## Running the program

```bash
cargo run -- --tasks tasks/example.toml
```

This writes `body.md`, `body.html`, and `subject.txt` in the current directory.
Another program can use these files to send email.
