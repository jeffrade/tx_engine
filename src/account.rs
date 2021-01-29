use rust_decimal::Decimal;

use crate::transaction::{Transaction, TransactionType};

#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    pub client: u16,
    pub available: Decimal,
    pub held: Decimal,
    pub total: Decimal,
    pub locked: bool,
}

impl Account {
    pub fn new(client: u16) -> Account {
        Account {
            client,
            available: Decimal::new(0, 0),
            held: Decimal::new(0, 0),
            total: Decimal::new(0, 0),
            locked: false,
        }
    }

    /// More performant if updating self instead of creating a new Account struct.
    pub fn update(tx: &Transaction, a: &Account) -> Account {
        let mut is_locked = false;
        let mut new_available = a.available;
        let mut new_held = a.held;

        if TransactionType::chargeback == tx.r#type {
            new_held = a.held - tx.amount;
            new_available = a.available - tx.amount;
            is_locked = true;
        } else if TransactionType::deposit == tx.r#type {
            new_available = a.available + tx.amount;
        } else if TransactionType::dispute == tx.r#type {
            new_held = a.held + tx.amount;
            new_available = a.available - tx.amount;
        } else if TransactionType::resolve == tx.r#type {
            new_held = a.held + tx.amount;
            new_available = a.available + tx.amount;
        } else if TransactionType::withdrawal == tx.r#type {
            new_available = a.available - tx.amount
        }

        Account {
            client: a.client,
            available: new_available,
            held: new_held,
            total: new_available + new_held,
            locked: is_locked,
        }
    }

    /// Workaround for dealing with RefCell in engine.rs
    pub fn copy(
        client: u16,
        available: Decimal,
        held: Decimal,
        total: Decimal,
        locked: bool,
    ) -> Account {
        Account {
            client,
            available,
            held,
            total,
            locked,
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_something() {
        assert!(true);
    }
}
