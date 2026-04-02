use clap::{Parser, Subcommand};
use strum_macros::Display;

use crate::net::{send_tcp, start_server};
mod commands;
mod net;
mod pipe;
mod utils;
#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    //This field is not a normal argument. It represents a subcommand.”
    command: Commands,
}
#[derive(Display, Debug, Subcommand)]
enum Commands {
    Send {
        #[arg()]
        file: Vec<String>,

        #[arg(long)]
        to: Option<String>,
    },
    //struct style enum
    Get {
        #[arg(long)]
        listen: Option<u16>,
    },
}
fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();
    // eprintln!("{:?}", cli);

    match cli.command {
        Commands::Send { file, to } => {
            if let Some(addr) = to {
                send_tcp(file, &addr)?;
            } else {
                commands::send::execute(file)?;
            }
        }
        Commands::Get { listen } => {
            if let Some(port) = listen {
                start_server(port)?;
            } else {
                commands::get::execute()?;
            }
        }
    }
    Ok(())
}
