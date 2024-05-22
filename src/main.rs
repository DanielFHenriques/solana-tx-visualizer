use solana_tx_visualizer::application::services::tracker_service::TrackerService;

#[tokio::main]
async fn main() {
    let cluster = "mainnet-beta";
    let tracker_service = TrackerService::new(cluster);

    tracker_service
        .track()
        .await
        .expect("Error tracking transactions!");
}
