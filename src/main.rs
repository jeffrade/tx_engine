use std::env;
use std::error::Error;
use std::io::ErrorKind;

extern crate csv;
#[macro_use]
extern crate serde_derive;

mod account;
mod csv_io;
mod engine;
mod transaction;

fn main() {
    init();
    match start() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("ERROR: {:?}", e);
        }
    };
}

fn start() -> Result<(), Box<dyn Error>> {
    let file_path: String = match get_arg(1) {
        Ok(file_path) => file_path,
        Err(e) => return Err(e),
    };
    match csv_io::read_and_process(&file_path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

/// Just a simple function to extract args out of `main()`.
/// Normally, I would introduce a lib like https://github.com/clap-rs/clap
/// but in the interest of time, this will suffice for now.
fn get_arg(position: usize) -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if position >= args.len() {
        Err(Box::new(std::io::Error::new(
            ErrorKind::Other,
            "You did not provide enough arguments!",
        )))
    } else {
        Ok(args[position].to_string())
    }
}

fn init() {
    // Clean up error file from last run, ignore if not present or
    // open (if open, will correctly Err downstream).
    if std::fs::remove_file("errors.log").is_ok() {}
}
