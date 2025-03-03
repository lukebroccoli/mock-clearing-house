use std::{collections::HashMap, sync::{Arc, Mutex}};
use crate::{
    accounts::{Account, AccountBalanceResponse, CreateAccountRequest},
    transactions::Transaction,
    AppError,
};

pub struct ClearingHouse {
    accounts: Arc<Mutex<HashMap<String, f64>>>,
}

impl ClearingHouse {
    pub fn new(accounts: Arc<Mutex<HashMap<String, f64>>>) -> Self {
        Self { accounts }
    }

    pub fn create_account(&self, request: CreateAccountRequest) -> Result<Account, AppError> {
        // Validate balance
        if request.balance < 0.0 {
            return Err(AppError::InvalidBalance);
        }

        let mut accounts = self.accounts.lock().unwrap();
        
        // Check if account already exists
        if accounts.contains_key(&request.id) {
            return Err(AppError::AccountAlreadyExists);
        }
        
        // Create account
        accounts.insert(request.id.clone(), request.balance);
        
        Ok(Account {
            id: request.id,
            balance: request.balance,
        })
    }

    pub fn get_account_balance(&self, id: &str) -> Result<AccountBalanceResponse, AppError> {
        let accounts = self.accounts.lock().unwrap();
        
        match accounts.get(id) {
            Some(balance) => Ok(AccountBalanceResponse { balance: *balance }),
            None => Err(AppError::AccountNotFound),
        }
    }

    pub fn process_transaction(&self, transaction: Transaction) -> Result<(), AppError> {
        // Validate transaction
        if transaction.amount <= 0.0 {
            return Err(AppError::InvalidAmount);
        }
        
        if transaction.sender == transaction.receiver {
            return Err(AppError::SenderReceiverIdentical);
        }
        
        // Lock the accounts for atomic update
        let mut accounts = self.accounts.lock().unwrap();
        
        // Check if both accounts exist
        let sender_balance = accounts.get(&transaction.sender).ok_or(AppError::AccountNotFound)?;
        if !accounts.contains_key(&transaction.receiver) {
            return Err(AppError::AccountNotFound);
        }
        
        // Check if sender has sufficient funds
        if *sender_balance < transaction.amount {
            return Err(AppError::InsufficientFunds);
        }
        
        // Update balances
        *accounts.get_mut(&transaction.sender).unwrap() -= transaction.amount;
        *accounts.get_mut(&transaction.receiver).unwrap() += transaction.amount;
        
        Ok(())
    }
}