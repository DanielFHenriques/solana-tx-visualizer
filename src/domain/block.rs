use crate::domain::transaction::Transaction;

pub struct Block {
    pub blockhash: String,
    pub transactions: Option<Vec<Transaction>>,
}

impl Block {
    pub fn new(blockhash: String, transactions: Option<Vec<Transaction>>) -> Self {
        Self {
            blockhash,
            transactions,
        }
    }
}
