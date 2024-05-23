use clap::Parser;
use solana_tx_visualizer::application::services::block_service::BlockService;
use solana_tx_visualizer::application::services::track_service::TrackService;
use solana_tx_visualizer::cli::{Cli, Commands};
use solana_tx_visualizer::infrastructure::gateways::block_gateway::BlockGatewayImpl;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Track { cluster } => {
            let block_gateway = BlockGatewayImpl::new(cluster);
            let tracker_service = TrackService::new(block_gateway);

            tracker_service
                .track()
                .await
                .expect("Error tracking transactions!");
        }
        Commands::Block { cluster, block_id } => {
            let block_gateway = BlockGatewayImpl::new(cluster);
            let block_service = BlockService::new(block_gateway);

            block_service.get_by_id(block_id);
        }
    }
}
