use chrono::Utc;
use serde_json::json;
use sha2::{Digest, Sha256};

use crate::transaction::Transaction;

/// Represents a block in the blockchain
///
/// Note: As this implementation is simple and do not use cryptography, there is no 'nonce' field
#[derive(Debug, Clone)]
pub struct Block {
    pub id: i64,
    pub created_at: i64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    /// Create a new block with the given transactions and the previous block
    pub fn new(previous_block: &Block, transactions: Vec<Transaction>) -> Block {
        let previous_hash = previous_block.hash.clone();
        let created_at = Utc::now().timestamp_millis();
        let id = previous_block.id + 1;
        let hash = Self::hash(id, created_at, transactions.clone(), previous_hash.clone());

        Block {
            id,
            created_at,
            transactions,
            previous_hash,
            hash,
        }
    }

    /// Create a new genesis block
    pub fn new_genesis() -> Block {
        let id = 0;
        let created_at = Utc::now().timestamp_millis();
        let transactions = vec![];
        let previous_hash = "000".to_string();
        let hash = Self::hash(id, created_at, transactions.clone(), previous_hash.clone());

        Block {
            id,
            created_at,
            transactions,
            previous_hash,
            hash,
        }
    }

    /// Check if the block is valid
    pub fn is_valid(&self, previous_block: &Block) -> bool {
        self.previous_hash == previous_block.hash
            && self.hash
                == Self::hash(
                    self.id,
                    self.created_at,
                    self.transactions.clone(),
                    self.previous_hash.clone(),
                )
            && self.id == previous_block.id + 1
    }

    /// Given the block data, calculate the hash as a string
    fn hash(
        id: i64,
        created_at: i64,
        transactions: Vec<Transaction>,
        previous_hash: String,
    ) -> String {
        let data = json!({
            "id": id,
            "created_at": created_at,
            "transactions": transactions,
            "previous_hash": previous_hash,
        });

        let mut hasher = Sha256::new();
        hasher.update(data.to_string().as_bytes());
        hasher
            .finalize()
            .as_slice()
            .to_owned()
            .iter()
            .fold(String::new(), |acc, byte| acc + &format!("{:02x}", byte))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_genesis() {
        let block = Block::new_genesis();

        assert_eq!(block.id, 0);
        assert_eq!(block.transactions.len(), 0);
        assert_eq!(block.previous_hash, "000");
        assert_eq!(block.hash.len(), 64);
    }

    #[test]
    fn test_new() {
        let genesis = Block::new_genesis();
        let block = Block::new(&genesis, vec![]);

        assert_eq!(block.id, 1);
        assert_eq!(block.transactions.len(), 0);
        assert_eq!(block.previous_hash, genesis.hash);
        assert_eq!(block.hash.len(), 64);
    }

    #[test]
    fn test_is_valid() {
        let genesis = Block::new_genesis();
        let mut block = Block::new(&genesis, vec![]);

        // Valid block
        assert!(block.is_valid(&genesis));

        // Invalid id
        block.id = 2;
        assert_eq!(block.is_valid(&genesis), false);

        // Invalid hash
        block = Block::new(&genesis, vec![]);
        block.hash = "invalid".to_string();
        assert_eq!(block.is_valid(&genesis), false);

        // Invalid previous hash
        block = Block::new(&genesis, vec![]);
        block.previous_hash = "invalid".to_string();
        assert_eq!(block.is_valid(&genesis), false);
    }

    #[test]
    fn test_hash() {
        let hash = Block::hash(1, 2, vec![], "000".to_string());

        assert_eq!(hash.len(), 64);
        assert_eq!(
            hash,
            "44959c0b295fde2d95df31b99e7f51f2b4727d64fd9fa654211ad27421577bb1"
        )
    }
}
