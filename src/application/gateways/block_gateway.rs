use crate::domain::block::Block;

pub trait BlockGateway {
    fn get_latest_block(&self) -> Result<Block, String>;
}
