use crate::application::gateways::block_gateway::BlockGateway;
use crate::domain::block::Block;

pub struct BlockService<G: BlockGateway> {
    gateway: G,
}

impl<G: BlockGateway> BlockService<G> {
    pub fn new(gateway: G) -> Self {
        Self { gateway }
    }

    pub async fn get_latest_block(&self) -> Result<Block, String> {
        return self.gateway.get_latest_block();
    }
}
