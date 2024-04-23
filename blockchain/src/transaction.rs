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

impl Transaction {
    /// Create a new account creation transaction
    pub fn account_creation(account_id: String, initial_balance: i64) -> Transaction {
        Transaction::AccountCreation(AccountCreation {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: chrono::Utc::now().timestamp_millis(),
            account_id,
            initial_balance,
        })
    }

    /// Create a new transfer transaction
    pub fn transfer(from_account: String, to_account: String, amount: i64) -> Transaction {
        Transaction::Transfer(Transfer {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: chrono::Utc::now().timestamp_millis(),
            from_account,
            to_account,
            amount,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_account_creation() {
        let transaction = Transaction::account_creation("account_id".to_string(), 100);

        match transaction {
            Transaction::AccountCreation(account_creation) => {
                assert_eq!(account_creation.account_id, "account_id");
                assert_eq!(account_creation.initial_balance, 100);
            }
            _ => panic!("Invalid transaction type"),
        }
    }

    #[test]
    fn test_transfer() {
        let transaction =
            Transaction::transfer("from_account".to_string(), "to_account".to_string(), 100);

        match transaction {
            Transaction::Transfer(transfer) => {
                assert_eq!(transfer.from_account, "from_account");
                assert_eq!(transfer.to_account, "to_account");
                assert_eq!(transfer.amount, 100);
            }
            _ => panic!("Invalid transaction type"),
        }
    }
}
