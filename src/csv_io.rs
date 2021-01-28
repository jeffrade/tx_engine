use csv::{DeserializeRecordsIter, Reader};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::ErrorKind;
use transaction::Transaction;

use crate::transaction;

pub fn read(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut reader = Reader::from_path(file_path)?;
    match process_transactions(reader.deserialize::<Transaction>()) {
        Ok(transactions) => Ok(transactions),
        Err(e) => Err(e),
    }
}

fn process_transactions(
    row_itr: DeserializeRecordsIter<File, Transaction>,
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut results: Vec<String> = Vec::new();
    let mut errors: Vec<String> = Vec::new();
    for row in row_itr {
        let tx: Option<Transaction> = match row {
            Ok(row) => Some(row),
            Err(e) => {
                errors.push(e.to_string());
                None
            }
        };
        if tx.is_some() {
            results.push(format!("{:?}", tx.unwrap()));
        }
    }
    match errors.len() {
        0 => Ok(results),
        _ => {
            log_errors(errors);
            Err(Box::new(std::io::Error::new(
                ErrorKind::Other,
                "There are errors processing the csv file - see errors.log!",
            )))
        }
    }
}

/// Just a quick-and-dirty function to write errors.log file and .expect()'ing on commands.
fn log_errors(errors: Vec<String>) {
    let mut file = File::create("errors.log").expect("Could not open error file!");
    for error in errors {
        file.write_all(error.as_bytes())
            .expect("Could not write error to file!");
        file.write_all(b"\n")
            .expect("Could not write error to file!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        assert!(true);
    }
}
