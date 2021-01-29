use rust_decimal::Decimal;
use std::cmp::PartialEq;

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    pub r#type: TransactionType,
    pub client: u16,
    pub tx: u32,
    pub amount: Decimal,
}

/// Allow non camel case to clear warnings, but normally when reading the csv file,
/// String.to_lowercase() is preferred. In the interest of time, this allows me to
/// let the csv lib to deserialize out-of-the-box.
#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum TransactionType {
    chargeback,
    deposit,
    dispute,
    resolve,
    withdrawal,
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_something() {
        assert!(true);
    }
}
