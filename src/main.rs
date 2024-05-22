use anyhow::Result;
use futures_util::StreamExt;
use solana_client::rpc_response::SlotUpdate;
use solana_pubsub_client::nonblocking::pubsub_client::PubsubClient;
use solana_tx_visualizer::application::gateways::block_gateway::BlockGateway;
use solana_tx_visualizer::infrastructure::gateways::block_gateway::BlockGatewayImpl;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc::unbounded_channel;

pub async fn watch(env: &str) -> Result<()> {
    let rpc_url = format!("https://api.{env}.solana.com");
    let websocket_url = format!("wss://api.{env}.solana.com/");

    let block_gateway = BlockGatewayImpl::new(rpc_url.as_str());

    // Subscription tasks will send a ready signal when they have subscribed.
    let (ready_sender, mut ready_receiver) = unbounded_channel::<()>();

    // Channel to receive unsubscribe channels (actually closures).
    // These receive a pair of `(Box<dyn FnOnce() -> BoxFuture<'static, ()> + Send>), &'static str)`,
    // where the first is a closure to call to unsubscribe, the second is the subscription name.
    let (unsubscribe_sender, mut unsubscribe_receiver) = unbounded_channel::<(_, &'static str)>();

    // The `PubsubClient` must be `Arc`ed to share it across tasks.
    let pubsub_client = Arc::new(PubsubClient::new(websocket_url.as_str()).await?);

    let mut join_handles = vec![];

    join_handles.push((
        "slot",
        tokio::spawn({
            // Clone things we need before moving their clones into the `async move` block.
            //
            // The subscriptions have to be made from the tasks that will receive the subscription messages,
            // because the subscription streams hold a reference to the `PubsubClient`.
            // Otherwise we would just subscribe on the main task and send the receivers out to other tasks.

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
                            std::thread::sleep(std::time::Duration::from_secs(5));
                        }
                    }
                }

                // This type hint is necessary to allow the `async move` block to use `?`.
                Ok::<_, anyhow::Error>(())
            }
        }),
    ));

    // Drop these senders so that the channels can close
    // and their receivers return `None` below.
    drop(ready_sender);
    drop(unsubscribe_sender);

    // Wait until all subscribers are ready before proceeding with application logic.
    while let Some(_) = ready_receiver.recv().await {}

    // Do application logic here.

    // Wait for input or some application-specific shutdown condition.
    tokio::io::stdin().read_u8().await?;

    // Unsubscribe from everything, which will shutdown all the tasks.
    while let Some((unsubscribe, _name)) = unsubscribe_receiver.recv().await {
        unsubscribe().await
    }

    // Wait for the tasks.
    for (_name, handle) in join_handles {
        let _ = handle.await;
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    watch("mainnet-beta").await.unwrap();
}
