use crate::application::gateways::block_gateway::BlockGateway;

pub struct BlockService<G> {
    block_gateway: G,
}

impl<G: BlockGateway> BlockService<G> {
    pub fn new(block_gateway: G) -> Self {
        Self { block_gateway }
    }

    pub fn get_by_id(&self, id: u64) -> () {
        match self.block_gateway.get_block(id) {
            Ok(block) => {
                println!("------------------------------------------------------------");
                println!("Latest block: {:?}", block.slot);
                for transaction in block.transactions {
                    println!(
                        "TX {:?} detected: {:?} sent {:?} USDC to {:?}",
                        transaction.signature,
                        transaction.source.address,
                        transaction.amount(),
                        transaction.destination.address
                    );
                }
            }
            _ => {
                println!("Error getting block by id!");
            }
        }
    }
}
