use clap::{Parser, Subcommand};


#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}


#[derive(Subcommand)]
pub enum Commands {
    Info,
    ConservationOn,
    ConservationOff,
}
