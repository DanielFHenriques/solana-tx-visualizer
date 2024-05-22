use crate::domain::transaction::Transaction;

#[derive(Debug)]
pub struct Block {
    pub blockhash: String,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(blockhash: String, transactions: Vec<Transaction>) -> Self {
        Self {
            blockhash,
            transactions,
        }
    }
}
