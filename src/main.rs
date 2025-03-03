mod accounts;
mod transactions;
mod clearing_house;

use accounts::{Account, AccountBalanceResponse, CreateAccountRequest};
use clearing_house::ClearingHouse;
use transactions::Transaction;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use std::{collections::HashMap, net::SocketAddr, sync::{Arc, Mutex}};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Custom error type
#[derive(Debug)]
enum AppError {
    AccountAlreadyExists,
    AccountNotFound,
    InvalidBalance,
    InvalidAmount,
    InsufficientFunds,
    SenderReceiverIdentical,
}

// Convert our custom errors to HTTP responses
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::AccountAlreadyExists => (StatusCode::CONFLICT, "Account already exists"),
            AppError::AccountNotFound => (StatusCode::NOT_FOUND, "Account not found"),
            AppError::InvalidBalance => (StatusCode::BAD_REQUEST, "Invalid balance (must be >= 0)"),
            AppError::InvalidAmount => (StatusCode::BAD_REQUEST, "Invalid amount (must be > 0)"),
            AppError::InsufficientFunds => (StatusCode::BAD_REQUEST, "Insufficient funds"),
            AppError::SenderReceiverIdentical => (StatusCode::BAD_REQUEST, "Sender and receiver cannot be the same"),
        };

        (status, Json(json!({ "error": error_message }))).into_response()
    }
}

// Application state that will be shared between handlers
#[derive(Clone)]
struct AppState {
    clearing_house: Arc<ClearingHouse>,
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create our clearing house with thread-safe storage
    let accounts = Arc::new(Mutex::new(HashMap::<String, f64>::new()));
    let clearing_house = Arc::new(ClearingHouse::new(accounts));
    
    // Build our application state
    let app_state = AppState { clearing_house };

    // Build our application with routes
    let app = Router::new()
        .route("/accounts", post(create_account))
        .route("/accounts/:id", get(get_account_balance))
        .route("/transactions", post(process_transaction))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    // Run our server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Handler for creating a new account
async fn create_account(
    State(state): State<AppState>,
    Json(request): Json<CreateAccountRequest>,
) -> Result<(StatusCode, Json<Account>), AppError> {
    let account = state.clearing_house.create_account(request)?;
    Ok((StatusCode::CREATED, Json(account)))
}

// Handler for getting an account balance
async fn get_account_balance(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<AccountBalanceResponse>, AppError> {
    let balance = state.clearing_house.get_account_balance(&id)?;
    Ok(Json(balance))
}

// Handler for processing a transaction
async fn process_transaction(
    State(state): State<AppState>,
    Json(transaction): Json<Transaction>,
) -> Result<StatusCode, AppError> {
    state.clearing_house.process_transaction(transaction)?;
    Ok(StatusCode::OK)
}