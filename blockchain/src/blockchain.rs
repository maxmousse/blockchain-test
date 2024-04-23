use crate::{block::Block, transaction::Transaction};

/// Represents a blockchain
#[derive(Debug, Clone)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    // Create a new blockchain with a genesis block
    pub fn new() -> Blockchain {
        let blocks = vec![Block::new_genesis()];
        Blockchain { blocks }
    }

    /// Given a list of transactions, add a new block to the blockchain
    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let previous_block = self
            .blocks
            .last()
            .expect("Blockchain should have at least one block");
        let block = Block::new(previous_block, transactions);
        self.blocks.push(block);
    }

    /// Check if the blockchain is valid
    pub fn is_valid(&self) -> bool {
        // Loop through all blocks
        self.blocks.iter().enumerate().all(|(i, block)| {
            // Skip genesis block
            if i == 0 {
                block.id == 0
            // Check if the block is valid
            } else {
                let previous_block = &self
                    .blocks
                    .get(i - 1)
                    .expect("Blockchain should have at least one block");
                block.is_valid(previous_block)
            }
        })
    }

    /// Check if an account exists in the blockchain
    pub fn account_exists(&self, account_id: &str) -> bool {
        // Loop through all transactions of all blocks
        // and check if there is an account creation transaction
        // with the given account_id
        self.blocks.iter().any(|block| {
            block
                .transactions
                .iter()
                .any(|transaction| match transaction {
                    Transaction::AccountCreation(account_creation) => {
                        account_creation.account_id == account_id
                    }
                    _ => false,
                })
        })
    }

    /// Get the balance of an account
    pub fn get_account_balance(&self, account_id: &str) -> i64 {
        // Loop through all transactions of all blocks
        self.blocks.iter().fold(0, |balance, block| {
            balance
                + block.transactions.iter().fold(
                    0,
                    |block_balance, transaction| match transaction {
                        Transaction::AccountCreation(account_creation) => {
                            // Add initial balance at account creation
                            if account_creation.account_id == account_id {
                                block_balance + account_creation.initial_balance
                            } else {
                                block_balance
                            }
                        }
                        Transaction::Transfer(transfer) => {
                            // Subtract amount if account is the sender
                            if transfer.from_account == account_id {
                                block_balance - transfer.amount
                            // Add amount if account is the receiver
                            } else if transfer.to_account == account_id {
                                block_balance + transfer.amount
                            } else {
                                block_balance
                            }
                        }
                    },
                )
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let blockchain = Blockchain::new();

        assert_eq!(blockchain.blocks.len(), 1);
        assert_eq!(blockchain.blocks[0].id, 0);
    }

    #[test]
    fn test_add_block() {
        let mut blockchain = Blockchain::new();
        blockchain.add_block(vec![]);

        assert_eq!(blockchain.blocks.len(), 2);
        assert_eq!(blockchain.blocks[1].id, 1);
        assert_eq!(
            blockchain.blocks[1].previous_hash,
            blockchain.blocks[0].hash
        );
    }

    #[test]
    fn test_is_valid() {
        // Valid
        let mut blockchain = Blockchain::new();
        blockchain.add_block(vec![]);
        assert!(blockchain.is_valid());

        // Invalid block id
        blockchain.blocks[1].id = 2;
        assert_eq!(blockchain.is_valid(), false);

        // Invalid block hash
        blockchain = Blockchain::new();
        blockchain.add_block(vec![]);
        blockchain.blocks[1].hash = "invalid".to_string();
        assert_eq!(blockchain.is_valid(), false);

        // Invalid previous hash
        blockchain = Blockchain::new();
        blockchain.add_block(vec![]);
        blockchain.blocks[1].previous_hash = "invalid".to_string();
        assert_eq!(blockchain.is_valid(), false);
    }

    #[test]
    fn test_account_exists() {
        let mut blockchain = Blockchain::new();
        blockchain.add_block(vec![Transaction::account_creation(
            "account_id".to_string(),
            100,
        )]);

        assert_eq!(blockchain.account_exists("account_id"), true);
        assert_eq!(blockchain.account_exists("Does not exist"), false);
    }

    #[test]
    fn test_get_account_balance() {
        let mut blockchain = Blockchain::new();
        blockchain.add_block(vec![
            Transaction::account_creation("account_1".to_string(), 100),
            Transaction::account_creation("account_2".to_string(), 500),
        ]);
        blockchain.add_block(vec![Transaction::transfer(
            "account_1".to_string(),
            "account_2".to_string(),
            50,
        )]);

        assert_eq!(blockchain.get_account_balance("account_1"), 50);
        assert_eq!(blockchain.get_account_balance("account_2"), 550);
    }
}
