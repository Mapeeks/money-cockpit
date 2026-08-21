use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: i64,
    pub title: String,
    pub bank_id: Option<i64>,
    pub account_type_id: Option<i64>,
    pub bic: Option<String>,
    pub iban: Option<String>,
    pub initial_balance: f64,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewAccount {
    pub title: String,
    pub bank_id: Option<i64>,
    pub account_type_id: Option<i64>,
    pub bic: Option<String>,
    pub iban: Option<String>,
    pub initial_balance: f64,
}
