use crate::application::gateways::block_gateway::BlockGateway;
use crate::domain::block::Block;
use solana_client::{rpc_client::RpcClient, rpc_config::RpcBlockConfig};

pub trait MaxSupportedTransactionVersion {
    fn new_with_max_supported_transaction_version(
        max_supported_transaction_version: &Option<u8>,
    ) -> Self;
}

impl MaxSupportedTransactionVersion for RpcBlockConfig {
    fn new_with_max_supported_transaction_version(
        max_supported_transaction_version: &Option<u8>,
    ) -> Self {
        Self {
            max_supported_transaction_version: *max_supported_transaction_version,
            ..Self::default()
        }
    }
}

pub struct BlockGatewayImpl {
    base_url: String,
}

impl BlockGatewayImpl {
    pub fn new<U: ToString>(base_url: U) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }
}

impl BlockGateway for BlockGatewayImpl {
    fn get_latest_block(&self) -> Result<Block, String> {
        let client = RpcClient::new(&self.base_url);
        let rpc_block_config =
            RpcBlockConfig::new_with_max_supported_transaction_version(&Some(0u8));

        let mut latest_slot = client.get_slot().expect("Failed to get slot");

        match client.get_block_with_config(latest_slot, rpc_block_config) {
            Ok(block) => {
                let b = Block::new(block.blockhash, None);
                return Ok(b);
            }
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }
}
