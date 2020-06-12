# Recurring tasks

![Build and Test](https://github.com/hockeybuggy/recurring_tasks/workflows/Build%20and%20Test/badge.svg)

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

## Cron Cheat sheet

```
  ┌───────────── sec (0 - 59)
  │ ┌───────────── minute (0 - 59)
  │ │ ┌───────────── hour (0 - 23)
  │ │ │ ┌───────────── day of the month (1 - 31)
  │ │ │ │ ┌───────────── month (1 - 12 or JAN-DEC)
  │ │ │ │ │ ┌───────────── day of the week (0 - 6 or SUN-SAT)
  │ │ │ │ │ │ ┌───────────── year
  │ │ │ │ │ │ │
  │ │ │ │ │ │ │
  * * * * * * *
```
