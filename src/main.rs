mod cli;
mod commands;

use clap::Parser;
use cli::{Cli, Commands};
use commands::{InitArgs, handle_init};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { 
            collection_name,
            directory,
            editor_opt,
            no_edit,
            verbose,
        } => {
            let args = InitArgs {
                collection_name,
                directory,
                editor_opt,
                no_edit,
                verbose,
            };

            handle_init(&args);
        }
    }
}
