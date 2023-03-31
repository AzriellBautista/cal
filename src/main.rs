use std::{process};
use std::io::Write;
use chrono::{Datelike, Local, Month, TimeZone};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() {
    let today = Local::now();
    let year = today.year();
    let month = today.month();
    let days_in_month = match get_days_in_month(year, month) {
        Ok(days) => days,
        Err(e) => {
            eprint!("Error: {}", e);
            process::exit(1);
        }
    };
    let month_name = match get_month_name(month) {
        Ok(month_name) => month_name,
        Err(e) => {
            eprint!("Error: {}", e);
            process::exit(1);
        }
    };
    let offset = get_first_day_of_month_offset(year, month);

    println!("{:^20}\nMo Tu We Th Fr Sa Su",
        format!("{} {}",
            month_name,
            year
        ),
    );

    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    for day in 1..=days_in_month + offset - 1 {
        if day < offset {
            write!(stdout, "   ").unwrap();
            continue;
        }

        if year == today.year() && month == today.month() && day - offset + 1 == today.day() {
            stdout.set_color(ColorSpec::new().set_bg(Some(Color::Red)).set_fg(Some(Color::White))).unwrap();
            write!(stdout, "{:>2} ", day - offset + 1).unwrap();
            stdout.reset().unwrap();
        } else {
            write!(stdout, "{:>2} ", day - offset + 1).unwrap();
        }


        if day % 7 == 0 {
            writeln!(stdout, "").unwrap();

        }
    }
    writeln!(stdout, "").unwrap();
}

fn get_days_in_month(
    year: i32,
    month: u32
) -> Result<u32, &'static str> {
    match month {
        1 | 3 | 5 | 7 | 9 | 11 => Ok(31),
        4 | 6 | 8 | 10 | 12 => Ok(30),
        2 => {
            if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                Ok(29)
            } else {
                Ok(28)
            }
        }
        _ => Err("Invalid month argument (only 1-12 is accepted).")
    }
}

fn get_month_name(
    month: u32
) -> Result<String, &'static str> {
    match month {
        1 => Ok(Month::January.name().to_string()),
        2 => Ok(Month::February.name().to_string()),
        3 => Ok(Month::March.name().to_string()),
        4 => Ok(Month::April.name().to_string()),
        5 => Ok(Month::May.name().to_string()),
        6 => Ok(Month::June.name().to_string()),
        7 => Ok(Month::July.name().to_string()),
        8 => Ok(Month::August.name().to_string()),
        9 => Ok(Month::September.name().to_string()),
        10 => Ok(Month::October.name().to_string()),
        11 => Ok(Month::November.name().to_string()),
        12 => Ok(Month::December.name().to_string()),
        _ => Err("Invalid month argument (only 1-12 is accepted).")
    }
} 

fn get_first_day_of_month_offset(
    year: i32,
    month: u32
) -> u32 {
    Local.with_ymd_and_hms(year, month, 1, 0, 0, 0)
        .unwrap()
        .weekday()
        .number_from_monday()
}