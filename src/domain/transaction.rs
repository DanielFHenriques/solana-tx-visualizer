use crate::domain::account::Account;
use crate::domain::program::Program;

#[derive(Debug)]
pub struct Transaction {
    pub signature: String,
    pub source: Account,
    pub destination: Account,
    pub program: Program,
    pub amount: f64,
    pub token: String,
}

impl Transaction {
    pub fn new(
        signature: String,
        source: Account,
        destination: Account,
        program: Program,
        amount: f64,
        token: String,
    ) -> Self {
        Self {
            signature: signature.to_string(),
            source,
            destination,
            program,
            amount,
            token: token.to_string(),
        }
    }
}
