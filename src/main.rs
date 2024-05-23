use clap::Parser;
use solana_tx_visualizer::application::services::tracker_service::TrackerService;
use solana_tx_visualizer::cli::{Cli, Commands};
use solana_tx_visualizer::infrastructure::gateways::block_gateway::BlockGatewayImpl;

#[tokio::main]
async fn main() {
    let cluster = "mainnet-beta";
    let block_gateway = BlockGatewayImpl::new(cluster);
    let tracker_service = TrackerService::new(block_gateway);

    tracker_service
        .track()
        .await
        .expect("Error tracking transactions!");
}
