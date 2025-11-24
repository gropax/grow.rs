use clap::{ArgAction, Parser, Subcommand};
use std::path::{PathBuf};

// Actions:
// - Create class schema
// - Validate class schema
// - Create class instance
// - Validate instances against class schema

#[derive(Debug, Parser)]
#[command(name = "grow")]
#[command(about = "A command line tool", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    Init {
        collection_name: String,

        #[arg(default_value = ".")]
        directory: PathBuf,

        #[arg(short = 'e', long = "editor")]
        editor_opt: Option<String>,

        #[arg(short = 'E', long = "no-edit")]
        no_edit: bool,

        #[arg(short, long, action = ArgAction::Count)]
        verbose: u8,
    },
}
