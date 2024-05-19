use solana_tx_visualizer::application::services::block_service::BlockService;
use solana_tx_visualizer::infrastructure::gateways::block_gateway::BlockGatewayImpl;

#[tokio::main]
async fn main() {
    let rpc_url = "https://api.mainnet-beta.solana.com";

    let gateway = BlockGatewayImpl::new(rpc_url);
    let service = BlockService::new(gateway);

    match service.get_latest_block().await {
        Ok(block) => {
            println!("Block Hash: {:?}", block.blockhash);
        }
        Err(e) => {
            println!("Failed to get block: {:?}", e);
        }
    }
}
