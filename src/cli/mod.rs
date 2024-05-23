use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Solana Transaction Tracker")]
#[command(about = "A CLI to track the transaction for a given mint address", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "track")]
    #[command(about = "Track transactions for a given cluster")]
    Track {
        #[arg(short, long, default_value = "mainnet-beta")]
        cluster: String,
    },
    Block {
        #[arg(short, long, default_value = "mainnet-beta")]
        cluster: String,
        #[arg(short, long)]
        block_id: u64,
    },
}
