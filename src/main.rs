mod cli;
mod lenovo;
mod info;
mod battery;

use clap::Parser;
use cli::Cli;

use crate::info::print_info;
use crate::cli::{Commands, ConservationAction};
use crate::lenovo::conservation::set_conservation;


fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Info => {
            print_info();
        }

        Commands::Conservation { action } => {
            match action {
                ConservationAction::On => {
                    set_conservation(true).unwrap();
                }

                ConservationAction::Off => {
                    set_conservation(false).unwrap();
                }
            }
        }
    }
}
