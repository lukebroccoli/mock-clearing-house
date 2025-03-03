# Clearing House System

A simplified clearing house system that simulates how financial transactions are processed between accounts, built with Rust and Axum.
Also rust is funny so the default branch is master not main.

Generated with claude. 
Todo:
- probably change the numbers to integers instead of floats
- 

## Overview

This system simulates a basic clearing house that acts as an intermediary between users performing financial transactions. It includes account creation, balance checking, and transaction processing with appropriate validations.

## Core Components

### clearing_house.rs

The `clearing_house.rs` file contains the core business logic of the system:

- **Data Storage**: Uses `Arc<Mutex<HashMap<String, f64>>>` for thread-safe in-memory storage
- **Account Creation**: Validates and creates new accounts with unique IDs
- **Balance Inquiries**: Retrieves current account balances
- **Transaction Processing**: Implements the critical clearing logic with these steps:
  1. Validates transaction parameters (positive amount, different sender/receiver)
  2. Locks the accounts data structure to ensure atomic operations
  3. Verifies both accounts exist and the sender has sufficient funds
  4. Updates both balances atomically (debiting the sender, crediting the receiver)
  5. Returns appropriate success/error responses

### main.rs

The `main.rs` file sets up the web server and API endpoints:

- **Server Configuration**: Configures and runs the Axum web server
- **Route Definitions**: Maps HTTP endpoints to handler functions
- **Error Handling**: Defines the `AppError` enum and converts errors to HTTP responses
- **API Handlers**: Implements three main handlers:
  1. `create_account`: Handles account creation requests
  2. `get_account_balance`: Retrieves account balances
  3. `process_transaction`: Processes transaction requests

## API Testing Guide

Below are all the curl commands to test the functionality of the clearing house system.

### 1. Creating Accounts

#### Create the first account
```bash
curl -X POST http://localhost:3000/accounts -H "Content-Type: application/json" -d '{"id": "user1", "balance": 1000.0}'
```
Expected output:
```
{"id":"user1","balance":1000.0}
```

#### Create a second account
```bash
curl -X POST http://localhost:3000/accounts -H "Content-Type: application/json" -d '{"id": "user2", "balance": 0.0}'
```
Expected output:
```
{"id":"user2","balance":0.0}
```

#### Test error: Create a duplicate account
```bash
curl -X POST http://localhost:3000/accounts -H "Content-Type: application/json" -d '{"id": "user1", "balance": 500.0}'
```
Expected output:
```
{"error":"Account already exists"}
```

#### Test error: Create an account with negative balance
```bash
curl -X POST http://localhost:3000/accounts -H "Content-Type: application/json" -d '{"id": "user3", "balance": -100.0}'
```
Expected output:
```
{"error":"Invalid balance (must be >= 0)"}
```

### 2. Checking Account Balances

#### Get user1's balance
```bash
curl http://localhost:3000/accounts/user1
```
Expected output:
```
{"balance":1000.0}
```

#### Get user2's balance
```bash
curl http://localhost:3000/accounts/user2
```
Expected output:
```
{"balance":0.0}
```

#### Test error: Get a non-existent user's balance
```bash
curl http://localhost:3000/accounts/nonexistent
```
Expected output:
```
{"error":"Account not found"}
```

### 3. Processing Transactions

#### Successful transaction: Transfer 500.0 from user1 to user2
```bash
curl -X POST http://localhost:3000/transactions -H "Content-Type: application/json" -d '{"sender": "user1", "receiver": "user2", "amount": 500.0}'
```
Expected output: No content (empty response with 200 OK status)

#### Verify user1's new balance
```bash
curl http://localhost:3000/accounts/user1
```
Expected output:
```
{"balance":500.0}
```

#### Verify user2's new balance
```bash
curl http://localhost:3000/accounts/user2
```
Expected output:
```
{"balance":500.0}
```

#### Test error: Insufficient funds
```bash
curl -X POST http://localhost:3000/transactions -H "Content-Type: application/json" -d '{"sender": "user1", "receiver": "user2", "amount": 1000.0}'
```
Expected output:
```
{"error":"Insufficient funds"}
```

#### Test error: Negative amount
```bash
curl -X POST http://localhost:3000/transactions -H "Content-Type: application/json" -d '{"sender": "user1", "receiver": "user2", "amount": -100.0}'
```
Expected output:
```
{"error":"Invalid amount (must be > 0)"}
```

#### Test error: Same sender and receiver
```bash
curl -X POST http://localhost:3000/transactions -H "Content-Type: application/json" -d '{"sender": "user1", "receiver": "user1", "amount": 100.0}'
```
Expected output:
```
{"error":"Sender and receiver cannot be the same"}
```

### 4. Additional Transaction Testing

#### Make another successful transaction
```bash
curl -X POST http://localhost:3000/transactions -H "Content-Type: application/json" -d '{"sender": "user1", "receiver": "user2", "amount": 100.0}'
```
Expected output: No content (empty response with 200 OK status)

#### Verify final balances

User1's balance:
```bash
curl http://localhost:3000/accounts/user1
```
Expected output:
```
{"balance":400.0}
```

User2's balance:
```bash
curl http://localhost:3000/accounts/user2
```
Expected output:
```
{"balance":600.0}
```

## Technical Notes

- **Thread Safety**: Uses `Arc<Mutex<HashMap>>` to ensure thread-safe access to account data
- **Atomic Operations**: Transaction processing locks the accounts to ensure both accounts are updated atomically
- **Data Type**: Uses `f64` for simplicity. In a production system, a decimal type would be more appropriate
- **In-Memory Storage**: Uses in-memory storage for simplicity. A production system would use a database

## Future Improvements

- Add persistence for accounts and transactions
- Implement proper decimal handling for financial calculations
- Add authentication and authorization
- Maintain transaction history
- Implement more complex clearing operations like netting