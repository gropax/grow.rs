use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "grow")]
#[command(about = "A command line tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Init {
        name: String,
    },

    Run {
    },
}


//#[derive(Parser, Debug)]
//#[command(version, about, long_about = None)]
//struct Args {
//    #[arg(short, long)]
//    name: String,
//
//    #[arg(short, long, default_value_t = 1)]
//    count: u8,
//}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name } => {
            println!("Init {name}")
        }
        Commands::Run { } => {
            println!("Run")
        }
    }
}
