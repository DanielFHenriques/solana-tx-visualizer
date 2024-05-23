use crate::domain::transaction::Transaction;

#[derive(Debug)]
pub struct Block {
    pub slot: u64,
    pub blockhash: String,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(slot: u64, blockhash: String) -> Self {
        Self {
            slot,
            blockhash,
            transactions: Vec::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        if transaction.amount() > 0.0 {
            self.transactions.push(transaction);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::account::Account;
    use crate::domain::program::Program;

    #[test]
    fn test_block_add_transaction() {
        let mut block = Block::new(0, "blockhash".to_string());
        let source = Account {
            address: "source".to_string(),
            index: 0,
            pre_balance: 100.0,
            post_balance: 50.0,
        };
        let destination = Account {
            address: "destination".to_string(),
            index: 1,
            pre_balance: 100.0,
            post_balance: 150.0,
        };
        let program = Program {
            address: "program".to_string(),
            index: 2,
        };

        let token = "USDC".to_string();
        let transaction =
            Transaction::new("signature".to_string(), source, destination, program, token);

        block.add_transaction(transaction.clone());
        assert_eq!(block.transactions.len(), 1);

        block.add_transaction(transaction.clone());
        assert_eq!(block.transactions.len(), 2);

        let source = Account {
            address: "source".to_string(),
            index: 0,
            pre_balance: 100.0,
            post_balance: 100.0,
        };
        let destination = Account {
            address: "destination".to_string(),
            index: 1,
            pre_balance: 100.0,
            post_balance: 100.0,
        };
        let program = Program {
            address: "program".to_string(),
            index: 2,
        };

        let token = "USDC".to_string();
        let transaction =
            Transaction::new("signature".to_string(), source, destination, program, token);

        block.add_transaction(transaction.clone());
        assert_eq!(block.transactions.len(), 2);
    }

    #[test]
    fn test_block_does_not_add_transaction() {
        let mut block = Block::new(0, "blockhash".to_string());
        let source = Account {
            address: "source".to_string(),
            index: 0,
            pre_balance: 100.0,
            post_balance: 100.0,
        };
        let destination = Account {
            address: "destination".to_string(),
            index: 1,
            pre_balance: 100.0,
            post_balance: 100.0,
        };
        let program = Program {
            address: "program".to_string(),
            index: 2,
        };

        let token = "USDC".to_string();
        let transaction =
            Transaction::new("signature".to_string(), source, destination, program, token);

        block.add_transaction(transaction.clone());
        assert_eq!(block.transactions.len(), 0);
    }
}
