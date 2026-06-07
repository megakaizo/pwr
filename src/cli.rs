use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Info,

    Conservation {
        #[command(subcommand)]
        action: ConservationAction,
    },
}

#[derive(Subcommand)]
pub enum ConservationAction {
    On,
    Off,
}
