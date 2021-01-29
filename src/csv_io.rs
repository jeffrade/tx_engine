use csv::{DeserializeRecordsIter, Reader};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::ErrorKind;

use crate::account::Account;
use crate::engine;
use crate::transaction::Transaction;

pub fn read_and_process(file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = Reader::from_path(file_path)?;
    match process_transactions(reader.deserialize::<Transaction>()) {
        Ok(transactions) => match engine::generate_client_accounts(transactions) {
            Ok(accounts) => write_accounts_output(accounts),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

/// This needs to be optimized for performance!
fn process_transactions(
    row_itr: DeserializeRecordsIter<File, Transaction>,
) -> Result<Vec<Transaction>, Box<dyn Error>> {
    let mut transactions: Vec<Transaction> = Vec::new();
    let mut errors: Vec<String> = Vec::new();
    for row in row_itr {
        // We're collecting errors and continuing to process rows knowing
        // that client accounts may be inaccurate. If `errors.log` is not
        // empty, use accounts.csv for debugging purposes only!
        match row {
            Ok(row) => {
                transactions.push(row);
            }
            Err(e) => {
                errors.push(e.to_string());
            }
        };
    }
    match errors.len() {
        0 => Ok(transactions),
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

/// Writes to stdout here
fn write_accounts_output(accounts: Vec<Account>) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_writer(std::io::stdout());
    for account in accounts {
        writer.serialize(account)?;
    }
    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_something() {
        assert!(true);
    }
}
