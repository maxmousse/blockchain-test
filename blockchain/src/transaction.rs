pub struct AccountCreation {
    pub id: String,
    pub created_at: u64,
    pub account_id: String,
    pub initial_balance: u64,
}

pub struct Transfer {
    pub id: String,
    pub created_at: u64,
    pub from_account: String,
    pub to_account: String,
    pub amount: u64,
}

pub enum Transaction {
    AccountCreation(AccountCreation),
    Transfer(Transfer),
}
