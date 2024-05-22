use crate::application::gateways::{block_gateway::BlockGateway, slot_gateway::SlotGateway};

pub struct TrackerService<G: BlockGateway, S: SlotGateway> {
    block_gateway: G,
    slot_gateway: S,
}

impl<G: BlockGateway, S: SlotGateway> TrackerService<G, S> {
    pub fn new(block_gateway: G, slot_gateway: S) -> Self {
        Self {
            block_gateway,
            slot_gateway,
        }
    }

    pub async fn track(&self) -> Result<(), String> {
        match self.slot_gateway.get_latest() {
            Ok(slot) => match self.block_gateway.get_block(slot) {
                Ok(_block) => {
                    return Ok(());
                }
                Err(e) => {
                    return Err(e);
                }
            },
            Err(e) => {
                return Err(e);
            }
        }
    }
}
