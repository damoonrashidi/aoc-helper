mod config;
mod creator;

use chrono::{Datelike, Utc};
use config::AdventConfig;
use inquire::{error::InquireResult, Editor, Select, Text};
use reqwest::header::COOKIE;

use crate::creator::Creator;

fn main() -> InquireResult<()> {
    let current_date = Utc::now();

    let latest_year = if current_date.month() < 12 {
        current_date.year() - 1
    } else {
        current_date.year()
    };

    let years = (2015..=latest_year).rev().map(|y| format!("{y}")).collect();
    let year = Select::new("What year", years).prompt()?;

    let latest_day = if year == current_date.year().to_string() {
        current_date.day()
    } else {
        24
    };

    let days = (1..=latest_day).rev().map(|d| format!("{d}")).collect();
    let day = Select::new("What day", days).prompt()?;

    let test_input = Editor::new("Test input")
        .with_formatter(&|_| String::from("..."))
        .prompt()?;
    let test_answer = Text::new("Test answer").prompt()?;

    let input = get_puzzle_input(&year, &day);

    let creator = Creator::new(AdventConfig::get());

    let _ = creator.code_file(&year, &day, &test_input, &test_answer);
    let _ = creator.input_file(&year, &day, &input);

    Ok(())
}

fn get_puzzle_input(year: &String, day: &String) -> String {
    let config = AdventConfig::get();

    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    reqwest::blocking::Client::new()
        .get(url)
        .header(COOKIE, format!("session={}", config.session.secret))
        .send()
        .and_then(reqwest::blocking::Response::text)
        .unwrap_or_default()
}
