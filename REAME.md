# Recurring tasks

This project sends reminders to myself on a schedule.

## The Plan

The intention is for this repo to run a program on a very simply cron that will
run at a normal interval (such as hourly). This cron will run a program that
checks a list of tasks which each have their own schedule. If that task will
next occur within the first cron's interval it will be included in a
notification.

## Running tests

```bash
cargo test
```
