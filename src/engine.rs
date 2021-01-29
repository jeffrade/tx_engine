use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;

use crate::account::Account;
use crate::transaction::Transaction;

pub fn generate_client_accounts(
    transactions: Vec<Transaction>,
) -> Result<Vec<Account>, Box<dyn Error>> {
    let mut client_account_map = HashMap::new();
    let mut accounts: Vec<Account> = Vec::new();

    for tx in transactions {
        let client_id: u16 = tx.client;
        client_account_map
            .entry(client_id)
            .or_insert_with(|| RefCell::new(Account::new(client_id)));

        let updated_account = match client_account_map.get_mut(&client_id) {
            Some(a) => {
                let current_account = a.borrow_mut();
                process_client_transaction(&tx, &current_account)
            }
            _ => Account::new(1), // Will never execute - workaround for `rustc --explain E0499`
        };
        client_account_map.insert(client_id, RefCell::new(updated_account));
    }

    for acc_ref in client_account_map.values() {
        let account = acc_ref.borrow();
        accounts.push(Account::copy(
            account.client,
            account.available,
            account.held,
            account.total,
            account.locked,
        ));
    }

    Ok(accounts)
}

fn process_client_transaction(tx: &Transaction, account: &Account) -> Account {
    Account::update(tx, account)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_something() {
        assert!(true);
    }
}
