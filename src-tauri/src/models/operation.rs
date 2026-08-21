use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Operation {
    pub id: i64,
    pub account_id: i64,
    pub label: String,
    pub date: String,
    pub operation_type: String,
    pub amount: f64,
    pub status: String,
    pub date_cleared: Option<String>,
    pub category_id: Option<i64>,
    pub assignment_id: Option<i64>,
    pub payee: Option<String>,
    pub project_id: Option<i64>,
    pub attachment: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewOperation {
    pub account_id: i64,
    pub label: String,
    pub date: String,
    pub operation_type: String,
    pub amount: f64,
    pub status: String,
    pub date_cleared: Option<String>,
    pub category_id: Option<i64>,
    pub assignment_id: Option<i64>,
    pub payee: Option<String>,
}
