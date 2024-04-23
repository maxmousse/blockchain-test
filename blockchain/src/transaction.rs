use serde::{Deserialize, Serialize};

/// Represents an account creation transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountCreation {
    pub id: String,
    pub created_at: i64,
    pub account_id: String,
    pub initial_balance: i64,
}

// Represents a transfer transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transfer {
    pub id: String,
    pub created_at: i64,
    pub from_account: String,
    pub to_account: String,
    pub amount: i64,
}

/// Represents a transaction that can be added to a block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Transaction {
    AccountCreation(AccountCreation),
    Transfer(Transfer),
}
