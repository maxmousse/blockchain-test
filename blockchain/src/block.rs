use crate::transaction::Transaction;

pub struct Block {
    pub id: u64,
    pub created_at: u64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
}
