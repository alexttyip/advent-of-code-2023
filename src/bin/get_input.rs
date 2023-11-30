use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::{env, fs};

use dotenv::dotenv;
use reqwest::header::USER_AGENT;
use reqwest::{blocking, header::COOKIE};

fn fetch_input(day: u8) -> String {
    let url = format!("https://adventofcode.com/2023/day/{day}/input");

    let session = env::var("SESSION").expect("AoC session ID must be set");

    let client = blocking::Client::new();

    let res = client
        .get(url)
        .header(COOKIE, format!("session={}", session))
        .header(
            USER_AGENT,
            "github.com/alexttyip/advent-of-code-2023 by tsztoyip@gmail.com",
        )
        .send()
        .expect("Error fetching input");

    if res.status().is_client_error() {
        panic!("Error fetching input - client error");
    }

    if res.status().is_server_error() {
        panic!("Error fetching input - server error");
    }

    res.text().expect("Error fetching input")
}

fn setup_day(day: u8) {
    let day_dir = format!("./src/bin/day{day:02}");

    if Path::new(day_dir.as_str()).exists() {
        println!("Day dir already exists");
        return;
    }

    let input = fetch_input(day);

    let rust_file = fs::read_to_string("./src/bin/template/main.rs")
        .unwrap()
        .replace("00", format!("{day:02}").as_str());

    fs::create_dir(&day_dir).expect("Error creating day dir");

    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(day_dir.to_string() + "/main.rs")
        .expect("Cannot open Rust file")
        .write_all(rust_file.as_bytes())
        .expect("Error writing input");

    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(day_dir + "/input.txt")
        .expect("Cannot open input file")
        .write_all(input.as_bytes())
        .expect("Error writing input");

    println!("Done! 🚀");
}

fn main() {
    dotenv().ok();

    let args: Vec<_> = env::args().collect();
    let day = args
        .get(1)
        .expect("Day number not found in argument")
        .parse()
        .expect("Argument is not a number");

    setup_day(day);
}
