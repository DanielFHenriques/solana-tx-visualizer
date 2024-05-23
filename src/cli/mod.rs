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
    #[command(about = "Track transactions for a given mint address")]
    Track {
        #[arg(short, long)]
        mint: String,
    },
}
