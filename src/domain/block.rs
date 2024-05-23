use crate::domain::transaction::Transaction;

#[derive(Debug)]
pub struct Block {
    pub slot: u64,
    pub blockhash: String,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(slot: u64, blockhash: String, transactions: Vec<Transaction>) -> Self {
        Self {
            slot,
            blockhash,
            transactions,
        }
    }
}
