use crate::infrastructure::gateways::block_gateway::BlockGatewayImpl;
use anyhow::Result;
use futures_util::StreamExt;
use solana_client::rpc_response::SlotUpdate;
use solana_pubsub_client::nonblocking::pubsub_client::PubsubClient;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc::unbounded_channel;

pub struct TrackerService {
    cluster: String,
}

impl TrackerService {
    pub fn new(cluster: &str) -> Self {
        Self {
            cluster: cluster.to_string(),
        }
    }

    pub async fn track(&self) -> Result<()> {
        let rpc_url = format!("https://api.{}.solana.com", &self.cluster);
        let websocket_url = format!("wss://api.{}.solana.com/", &self.cluster);
        let block_gateway = BlockGatewayImpl::new(rpc_url.as_str());
        let (ready_sender, mut ready_receiver) = unbounded_channel::<()>();
        let (unsubscribe_sender, mut unsubscribe_receiver) =
            unbounded_channel::<(_, &'static str)>();
        let pubsub_client = Arc::new(PubsubClient::new(websocket_url.as_str()).await?);

        let mut join_handles = vec![];

        join_handles.push((
            "slot",
            tokio::spawn({
                let ready_sender = ready_sender.clone();
                let unsubscribe_sender = unsubscribe_sender.clone();
                let pubsub_client = Arc::clone(&pubsub_client);
                async move {
                    let (mut slot_updates_notifications, slot_updates_unsubscribe) =
                        pubsub_client.slot_updates_subscribe().await?;

                    // With the subscription started,
                    // send a signal back to the main task for synchronization.
                    ready_sender.send(()).expect("channel");

                    // Send the unsubscribe closure back to the main task.
                    unsubscribe_sender
                        .send((slot_updates_unsubscribe, "slot"))
                        .map_err(|e| format!("{}", e))
                        .expect("channel");

                    // Drop senders so that the channels can close.
                    // The main task will receive until channels are closed.
                    drop((ready_sender, unsubscribe_sender));

                    // Do something with the subscribed messages.
                    // This loop will end once the main task unsubscribes.
                    while let Some(slot_info) = slot_updates_notifications.next().await {
                        if let SlotUpdate::Completed { slot, timestamp: _ } = slot_info {
                            if let Ok(block) = block_gateway.get_block(slot) {
                                println!(
                                    "------------------------------------------------------------"
                                );
                                println!("Latest block: {:?}", slot);
                                for transaction in block.transactions {
                                    println!(
                                        "TX detected: {:?} sent {:?} USDC to {:?}",
                                        transaction.source.address,
                                        transaction.amount,
                                        transaction.destination.address
                                    );
                                }
                            } else {
                                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                            }
                        }
                    }

                    Ok::<_, anyhow::Error>(())
                }
            }),
        ));

        drop(ready_sender);
        drop(unsubscribe_sender);

        while let Some(_) = ready_receiver.recv().await {}

        tokio::io::stdin().read_u8().await?;

        while let Some((unsubscribe, _name)) = unsubscribe_receiver.recv().await {
            unsubscribe().await
        }

        for (_name, handle) in join_handles {
            let _ = handle.await;
        }

        Ok(())
    }
}
