#[derive(Debug, Clone)]
pub struct Account {
    pub address: String,
    pub index: u8,
    pub pre_balance: f64,
    pub post_balance: f64,
}

impl Account {
    pub fn new<U: ToString>(address: U, index: u8, pre_balance: f64) -> Self {
        Self {
            address: address.to_string(),
            index,
            pre_balance,
            post_balance: pre_balance,
        }
    }

    pub fn update_post_balance(&mut self, post_balance: f64) {
        self.post_balance = post_balance;
    }
}
