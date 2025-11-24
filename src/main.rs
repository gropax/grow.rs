use clap::{Parser, Subcommand};
use directories_next::ProjectDirs;
use std::fs;
use std::env;
use std::path::PathBuf;

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
    Run {},
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init {
            class_name,
            directory,
            editor_opt,
            no_edit,
            verbose,
        } => {
            eprintln!("TODO: ensure data dir.");
            ensure_data_dir(verbose);
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

        Commands::Run {} => {
            eprintln!("Run")
        }
    }
}

fn resolve_editor(editor_opt: &Option<String>, verbose: bool) -> String {
    if let Some(editor) = editor_opt.as_deref() {
        if verbose {
            eprintln!("Using editor from CLI argument: {}", editor);
        }
        return editor.to_string();
    }

    if let Ok(editor) = env::var("VISUAL") {
        if verbose {
            eprintln!("Using editor from VISUAL environment variable: {}", editor);
        }
        return editor;
    }

    if let Ok(editor) = env::var("EDITOR") {
        if verbose {
            eprintln!("Using editor from EDITOR environment variable: {}", editor);
        }
        return editor;
    }

    if verbose {
        eprintln!("No editor specified, falling back to vim");
    }
    "vim".to_string()
}

fn ensure_data_dir(verbose: bool) -> std::path::PathBuf {
    let proj = ProjectDirs::from("", "", "grow")
        .expect("Could not determine standard project directory");

    let data_dir = proj.data_dir();

    if !data_dir.exists() {
        if let Err(e) = fs::create_dir_all(data_dir) {
            eprintln!(
                "ERROR: could not create data directory at {}: {}",
                data_dir.display(),
                e
            )
        }

        if verbose {
            eprintln!("INFO: Created directory {}", data_dir.display());
        }
    }

    data_dir.to_path_buf()
}






