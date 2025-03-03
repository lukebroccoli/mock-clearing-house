use serde::{Deserialize, Serialize};
//use crate::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub balance: f64,
}

#[derive(Debug, Deserialize)]
pub struct CreateAccountRequest {
    pub id: String,
    pub balance: f64,
}

#[derive(Debug, Serialize)]
pub struct AccountBalanceResponse {
    pub balance: f64,
}