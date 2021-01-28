use std::env;
use std::error::Error;
use std::io::ErrorKind;

extern crate csv;
#[macro_use]
extern crate serde_derive;

mod csv_io;
mod transaction;

fn main() {
    init();
    match start() {
        Ok(results) => println!("{:?}", results),
        Err(e) => {
            eprintln!("ERROR: {:?}", e);
        }
    };
}

fn start() -> Result<Vec<String>, Box<dyn Error>> {
    let file_path: String = match get_arg(1) {
        Ok(file_path) => file_path,
        Err(e) => return Err(e),
    };
    let results = match csv_io::read(&file_path) {
        Ok(results) => Ok(results),
        Err(e) => Err(e),
    };
    results
}

/// Just a simple function to extract args out of `main()`.
/// Normally, I would introduce a lib like https://github.com/clap-rs/clap
/// but in the interest of time, this will suffice for now.
fn get_arg(position: usize) -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if position >= args.len() {
        // Err(Error::new(
        //     ErrorKind::Other,
        //     "You did not provide enough arguments!",
        // ))
        Err(Box::new(std::io::Error::new(
            ErrorKind::Other,
            "You did not provide enough arguments!",
        )))
    } else {
        Ok(args[position].to_string())
    }
}

fn init() {
    // Clean up file from last run, ignore if not present or
    // open (if open, will correctly signal downstream).
    match std::fs::remove_file("errors.log") {
        Ok(_) => (),
        Err(_) => (),
    }
}
