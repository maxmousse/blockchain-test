use blockchain::{blockchain::Blockchain, transaction::Transaction};

/// Represents a node in the blockchain network.
pub struct Node {
    pub transaction_pool: Vec<Transaction>,
    pub blockchain: Blockchain,
}

impl Node {
    /// Creates a new node with an empty transaction pool and a fresh blockchain.
    pub fn new() -> Self {
        Node {
            transaction_pool: Vec::new(),
            blockchain: Blockchain::new(),
        }
    }

    /// Mines a new block with the transactions in the transaction pool.
    pub fn mine_block(&mut self) {
        self.blockchain.add_block(self.transaction_pool.clone());
        self.transaction_pool.clear();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let node = Node::new();

        assert_eq!(node.transaction_pool.len(), 0);
        assert_eq!(node.blockchain.blocks.len(), 1);
        assert_eq!(node.blockchain.blocks[0].id, 0);
    }

    #[test]
    fn test_mine_block() {
        let mut node = Node::new();
        node.transaction_pool
            .push(Transaction::account_creation("account_id".to_string(), 100));
        assert_eq!(node.transaction_pool.len(), 1);

        node.mine_block();

        assert_eq!(node.transaction_pool.len(), 0);
        assert_eq!(node.blockchain.blocks.len(), 2);
        assert_eq!(node.blockchain.blocks[1].transactions.len(), 1);
    }
}
