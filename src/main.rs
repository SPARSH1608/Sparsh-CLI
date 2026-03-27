use clap::{Parser, Subcommand};
use strum_macros::Display;
mod commands;
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
    Send { file: Vec<String> },
    //struct style enum
    Get {},
}
fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();
    // eprintln!("{:?}", cli);

    match cli.command {
        Commands::Send { file } => commands::send::execute(file)?,
        Commands::Get {} => commands::get::execute()?,
    }
    Ok(())
}
