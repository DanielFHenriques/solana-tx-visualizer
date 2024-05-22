use solana_tx_visualizer::application::services::tracker_service::TrackerService;
use solana_tx_visualizer::infrastructure::gateways::block_gateway::BlockGatewayImpl;

#[tokio::main]
async fn main() {
    let cluster = "mainnet-beta";
    let rpc_url = format!("https://api.{cluster}.solana.com");
    let block_gateway = BlockGatewayImpl::new(rpc_url);
    let tracker_service = TrackerService::new(cluster, block_gateway);

    tracker_service
        .track()
        .await
        .expect("Error tracking transactions!");
}
