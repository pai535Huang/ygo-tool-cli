pub mod api;
pub mod cmd_img;
pub mod cmd_search;
pub mod cmd_fzf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "ygotool")]
#[command(about = "A CLI tool to search Yu-Gi-Oh! cards from ygocdb API", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Search for a card and print its information
    Search {
        /// The query string (card name, password, cid, or effect text)
        query: String,
    },
    /// Search for a card and print its image
    Img {
        /// The query string
        query: String,
    },
    #[command(hide = true)]
    FzfList {
        query: String,
    },
    #[command(hide = true)]
    FzfPreview {
        id: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Search { query }) => {
            if let Err(e) = cmd_search::run(query).await {
                eprintln!("Error: {}", e);
            }
        }
        Some(Commands::Img { query }) => {
            if let Err(e) = cmd_img::run(query).await {
                eprintln!("Error: {}", e);
            }
        }
        Some(Commands::FzfList { query }) => {
            if let Err(e) = cmd_fzf::run_list(query).await {
                eprintln!("Error: {}", e);
            }
        }
        Some(Commands::FzfPreview { id }) => {
            if let Err(e) = cmd_fzf::run_preview(id).await {
                eprintln!("Error: {}", e);
            }
        }
        None => {
            if let Err(e) = cmd_fzf::run_fzf().await {
                eprintln!("Error running fzf: {}", e);
            }
        }
    }
}
