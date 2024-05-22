use crate::domain::block::Block;

pub trait BlockGateway {
    fn get_block(&self, block: u64) -> Result<Block, String>;
}
