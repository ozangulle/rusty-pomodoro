# Rusty Pomodoro [![Build Status](https://travis-ci.org/ozangulle/rusty-pomodoro.svg?branch=master)](https://travis-ci.org/ozangulle/rusty-pomodoro)

## Why another pomodoro app

First of all, it is a great opportunity to learn and practice Rust. Furthermore, I always wanted to be able to track and analyze my pomodoro habits. As far as I know, there is no open source application available which provides an on-site record of the process.

## Current Features

Current available features:

- Manage the pomodoro cycle using the terminal user interface.
- The app logs your pomodoro process day by day in a csv file.
- The app picks up the number of pomodoros from where you left if you run the app multiple times on a given day.
- Customize the name of the record file name and location in the config file.

## Upcoming/Desired Features

- More control over the pomodoro process (pausing/stopping without exiting the app).
- Some kind of graphical interface at a later point.

## How to use

Simply get the binary for your system from "releases" and run it. rusty-pomodoro is currently a cli tool, so it will open the terminal to run.

Alternatively, you can compile it yourself by running ```cargo build --release```

### Customizations

You can customize the name of the record file (default is "pom-record.csv")
and its location (default location is the same directory as the executable).

In order to customize these values, you need to create a YAML file called
"rp-config.yml" in the same directory as the executable.

Current configuration template is as follows:

```
record_name = "{Name of the record file}" # e.g. "my_records"
record_location = "{Path for the filename without a slash at the end}" # e.g. "/home/ogulle/documents"
```


## License

MIT
