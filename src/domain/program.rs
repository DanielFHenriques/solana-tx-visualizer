#[derive(Debug, Clone)]
pub struct Program {
    pub address: String,
    pub index: u8,
}

impl Program {
    pub fn new<U: ToString>(address: U, index: u8) -> Self {
        Self {
            address: address.to_string(),
            index,
        }
    }
}
