use clap::{Parser, Subcommand, ArgAction};
use tracing::{debug, info, warn, error};
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

        #[arg(short, long, action = ArgAction::Count)]
        verbose: u8,
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
            init_tracing(verbose);
            ensure_data_dir();

            warn!("TODO: ensure datasets file.");

            warn!("TODO: create dir [{}/.grow].", directory.display());
            warn!("TODO: create entry for {class_name}.");

            warn!("TODO: create dir [./{class_name}].");
            warn!("TODO: create dir [./{class_name}/instances].");
            warn!("TODO: create file [./{class_name}/schema.yml].");

            if !no_edit {
                let editor = resolve_editor(&editor_opt);
                warn!("TODO: open file [./{class_name}/schema.yml] with {editor}.");
            }
        }

        Commands::Run {} => {
            warn!("Run")
        }
    }
}

fn init_tracing(level: u8) {
    let filter = match level {
        0 => "error",
        1 => "warn",
        2 => "info",
        _ => "debug",
    };

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .init();
}

fn resolve_editor(editor_opt: &Option<String>) -> String {
    if let Some(editor) = editor_opt.as_deref() {
        debug!("Using editor from CLI argument: {}", editor);
        return editor.to_string();
    }

    if let Ok(editor) = env::var("VISUAL") {
        debug!("Using editor from VISUAL environment variable: {}", editor);
        return editor;
    }

    if let Ok(editor) = env::var("EDITOR") {
        debug!("Using editor from EDITOR environment variable: {}", editor);
        return editor;
    }

    debug!("No editor specified, falling back to vim");
    "vim".to_string()
}

fn ensure_data_dir() -> std::path::PathBuf {
    let proj = ProjectDirs::from("", "", "grow")
        .expect("Could not determine standard project directory");

    let data_dir = proj.data_dir();

    if !data_dir.exists() {
        if let Err(e) = fs::create_dir_all(data_dir) {
            error!(
                "ERROR: could not create data directory at {}: {}",
                data_dir.display(),
                e
            )
        }

        info!("INFO: Created directory {}", data_dir.display());
    }

    data_dir.to_path_buf()
}






