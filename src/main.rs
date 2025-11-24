use clap::{ArgAction, Parser, Subcommand};
use directories_next::ProjectDirs;
use std::env;
use std::fs;
use std::path::{PathBuf, Path};
use tracing::{debug, error, info, warn};

// Actions:
// - Create class schema
// - Validate class schema
// - Create class instance
// - Validate instances against class schema

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
    Run {},
}

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
            init_tracing(verbose);

            ensure_data_dir();

            let collection_dir = directory.join(&collection_name);
            debug!("Using collection directory: {}", collection_dir.display());
            ensure_dir(&collection_dir);

            let documents_dir = collection_dir.join("documents");
            debug!("Using documents directory: {}", documents_dir.display());
            ensure_dir(&documents_dir);

            warn!("TODO: create file [./{collection_name}/schema.yml].");

            warn!("TODO: create entry for {collection_name}.");

            if !no_edit {
                let editor = resolve_editor(&editor_opt);
                warn!("TODO: open file [./{collection_name}/schema.yml] with {editor}.");
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

fn ensure_data_dir() -> PathBuf {
    let proj =
        ProjectDirs::from("", "", "grow").expect("Could not determine standard project directory");

    let data_dir = proj.data_dir();
    debug!("Using data directory: {}", data_dir.display());
    ensure_dir(data_dir);

    data_dir.to_path_buf()
}

fn ensure_dir(dir_path: &Path) {
    if !dir_path.exists() {
        if let Err(e) = fs::create_dir_all(dir_path) {
            error!(
                "ERROR: could not create data directory at {}: {}",
                dir_path.display(),
                e
            );
        }

        info!("INFO: Created directory {}", dir_path.display());
    }
}



