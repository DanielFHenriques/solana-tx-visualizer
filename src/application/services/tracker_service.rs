use crate::application::gateways::block_gateway::BlockGateway;
use crate::domain::block::Block;
use anyhow::Result;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc::unbounded_channel;

pub struct TrackerService<G> {
    block_gateway: G,
}

impl<G: BlockGateway + Clone + Send + Sync + 'static> TrackerService<G> {
    pub fn new(block_gateway: G) -> Self {
        Self { block_gateway }
    }

    pub async fn track(&self) -> Result<()> {
        let (ready_sender, mut ready_receiver) = unbounded_channel::<()>();
        let (unsubscribe_sender, mut unsubscribe_receiver) = unbounded_channel::<_>();
        let (block_update_sender, mut block_update_receiver) = unbounded_channel::<Block>();
        let block_gateway = Arc::new(self.block_gateway.clone());

        block_gateway
            .subscribe(&ready_sender, &unsubscribe_sender, &block_update_sender)
            .await?;

        drop(ready_sender);
        drop(unsubscribe_sender);
        drop(block_update_sender);

        while let Some(_) = ready_receiver.recv().await {}

        while let Some(block) = block_update_receiver.recv().await {
            println!("------------------------------------------------------------");
            println!("Latest block: {:?}", block.slot);
            for transaction in block.transactions {
                println!(
                    "TX detected: {:?} sent {:?} USDC to {:?}",
                    transaction.source.address, transaction.amount, transaction.destination.address
                );
            }
        }

        tokio::io::stdin().read_u8().await?;

        while let Some(unsubscribe) = unsubscribe_receiver.recv().await {
            unsubscribe().await
        }

        return Ok(());
    }
}
