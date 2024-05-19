use solana_sdk::message::v0::Message;

pub struct Transaction {
    pub signature: String,
    pub message: Message,
}

impl Transaction {
    pub fn new(signature: String, message: Message) -> Self {
        Self { signature, message }
    }
}
