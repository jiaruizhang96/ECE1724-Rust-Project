use clap::{Parser, Subcommand};
use std::error::Error;

#[derive(Parser)]
#[command(name = "Distributed File Storage CLI")]
#[command(about = "A CLI for interacting with the distributed file storage system", long_about = None)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

// the supported commands are as follows
#[derive(Subcommand)]
enum Commands {
    /// Connect to the network
    Connect {
        /// Node address to connect to
        #[arg(short, long)]
        address: String,
    },
    /// Upload a file
    Upload {
        /// Path to the file to upload
        #[arg(short, long)]
        file: String,
    },
    /// Download a file
    Download {
        /// Filename to download
        #[arg(short, long)]
        filename: String,
        /// Destination path
        #[arg(short, long)]
        destination: String,
    },
}

pub async fn run_cli() -> Result<(), Box<dyn Error>> {
    let cli = CLI::parse();

    match &cli.command {
        Commands::Connect { address } => {
            println!("Connecting to the network at {}", address);
            // TODO: implement connection logic here
        }
        Commands::Upload { file } => {
            println!("Uploading file: {}", file);
            // TODO: implement upload logic here
        }
        Commands::Download { filename, destination } => {
            println!("Downloading file: {} to {}", filename, destination);
            // TODO: implement download logic here
        }
    }

    Ok(())
}
