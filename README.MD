# cal (v1.0)

## Information

Displays calendar on the terminal. A **replica** of the `cal` command in Linux.
Accepts month and year options and displays the calendar for that month and/or 
year. If no options are used, it defaults to displaying the calendar for the 
current month. Highlights the current day if present on the current calendar. 
It is 100% written on Rust.

This project is a self-project created initially to replicate the `cal` command
in Linux over to Windows. I have just started on learning Rust and I decided to
create this project to apply and demonstrate what I have learned in Rust. The 
code may not be the most efficient and may have unhandled or poorly handle 
panics.

## Usage

```sh
cal [-m, --month <value>] [-y, --year <value>]
```

## Sample Output

```
     April 2023
Mo Tu We Th Fr Sa Su
                1  2
 3  4  5  6  7  8  9
10 11 12 13 14 15 16
17 18 19 20 21 22 23
24 25 26 27 28 29 30
```

## Dependencies

Crates used:
- chrono `0.4.24`
- termcolor `1.2.0`
- clap `4.2.1`

## Build

Run `cargo run` or `cargo build --release` to build the project as an executable.

## To-do's

- [x] Option to change month and year.
- [ ] Option to change order of week days (e.g. the week starts on Sunday).
- [ ] Option to toggle highlighting of current day so that it can be compatible
      with the redirect operation to file (`>`` or `>>`) on terminal.
- [ ] Option to display multiple calendars given a date range.
- [ ] Option to change color of weekends to differentiate the from weekdays.

## Hotel

Trivago