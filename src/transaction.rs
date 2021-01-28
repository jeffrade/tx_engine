use rust_decimal::Decimal;

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    r#type: TransactionType,
    client: u16,
    tx: u32,
    amount: Decimal,
}

/// Allow non camel case to clear warnings, but normally when reading the csv file,
/// String.to_lowercase() is preferred. In the interest of time, this allows me to
/// let the csv lib to deserialize out-of-the-box.
#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize)]
pub enum TransactionType {
    chargeback,
    deposit,
    dispute,
    resolve,
    withdrawal,
}
