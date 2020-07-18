# Recurring tasks

![Build and Test](https://github.com/hockeybuggy/recurring_tasks/workflows/Build%20and%20Test/badge.svg)

This project sends reminders to myself on a schedule.


## How this works

This repo works using a counterpart private repository. This repository's
concern is related to finding upcoming tasks given a task file of a specific
format.

The counterpart private repository will use this repository, but it's concerns
are related to periodically running and notifying someone..


## Running tests

```bash
cargo test
```


## Running the program

```bash
cargo run -- --tasks tasks/example.toml
```

This will output a `body.md`, `body.html` and a `subject.txt`. This files are
then used for sending emails.
