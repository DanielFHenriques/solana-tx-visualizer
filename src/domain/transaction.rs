use crate::domain::account::Account;
use crate::domain::program::Program;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub signature: String,
    pub source: Account,
    pub destination: Account,
    pub program: Program,
    pub token: String,
}

impl Transaction {
    pub fn new(
        signature: String,
        source: Account,
        destination: Account,
        program: Program,
        token: String,
    ) -> Self {
        Self {
            signature: signature.to_string(),
            source,
            destination,
            program,
            token: token.to_string(),
        }
    }

    pub fn amount(&self) -> f64 {
        self.destination.post_balance - self.destination.pre_balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_amount() {
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

        assert_eq!(transaction.amount(), 50.0);
    }
}
