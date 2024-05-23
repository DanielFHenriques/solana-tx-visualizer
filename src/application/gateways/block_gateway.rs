use crate::domain::block::Block;
use anyhow::Result;
use futures_util::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;

pub trait BlockGateway {
    async fn subscribe(
        self: Arc<Self>,
        ready_sender: &UnboundedSender<()>,
        unsubscribe_sender: &UnboundedSender<
            Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>,
        >,
        block_update_sender: &UnboundedSender<Block>,
    ) -> Result<()>;
    fn get_block(&self, block: u64) -> Result<Block, String>;
}
