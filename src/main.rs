use std::env;
use std::path::PathBuf;
use clap::{Parser, Subcommand};

// Actions:
// - Create class schema
// - Validate class schema
// - Create class instance
// - Validate instances against class schema

// grow class
//   - create
//   - remove (?)
//   - check
// grow inst[ance]

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
        class_name: String,

        #[arg(default_value = ".")]
        directory: PathBuf,

        #[arg(short = 'e', long = "editor")]
        editor_opt: Option<String>,

        #[arg(short = 'E', long = "no-edit")]
        no_edit: bool,

        #[arg(short = 'v', long = "verbose")]
        verbose: bool,
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
        Commands::Init { class_name, directory, editor_opt, no_edit, verbose } => {
            eprintln!("TODO: ensure data dir.");
            eprintln!("TODO: ensure datasets file.");

            eprintln!("TODO: create dir [{}/.grow].", directory.display());
            eprintln!("TODO: create entry for {class_name}.");

            eprintln!("TODO: create dir [./{class_name}].");
            eprintln!("TODO: create dir [./{class_name}/instances].");
            eprintln!("TODO: create file [./{class_name}/schema.yml].");

            if !no_edit {
                let editor = resolve_editor(&editor_opt, verbose);
                eprintln!("TODO: open file [./{class_name}/schema.yml] with {editor}.");
            }
        }
        Commands::Run { } => {
            eprintln!("Run")
        }
    }
}


fn resolve_editor(editor_opt: &Option<String>, verbose: bool) -> String {
    if let Some(editor) = editor_opt.as_deref() {
        if verbose {
            eprintln!("Using editor from CLI argument: {}", editor);
        }
        return editor.to_string()
    }

    if let Ok(editor) = env::var("VISUAL") {
        if verbose {
            eprintln!("Using editor from VISUAL environment variable: {}", editor);
        }
        return editor
    }

    if let Ok(editor) = env::var("EDITOR") {
        if verbose {
            eprintln!("Using editor from EDITOR environment variable: {}", editor);
        }
        return editor
    }

    if verbose {
        eprintln!("No editor specified, falling back to vim");
    }
    "vim".to_string()
}



