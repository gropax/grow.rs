use directories_next::ProjectDirs;
use std::env;
use std::fs;
use std::path::{PathBuf, Path};
use tracing::{debug, error, info, warn};

#[derive(Debug)]
pub struct InitArgs {
    pub collection_name: String,
    pub directory: PathBuf,
    pub editor_opt: Option<String>,
    pub no_edit: bool,
    pub verbose: u8,
}

pub fn handle_init(args: &InitArgs) {
    init_tracing(args.verbose);

    ensure_data_dir();

    let collection_dir = args.directory.join(&args.collection_name);
    debug!("Using collection directory: {}", collection_dir.display());
    ensure_dir(&collection_dir);

    let documents_dir = collection_dir.join("documents");
    debug!("Using documents directory: {}", documents_dir.display());
    ensure_dir(&documents_dir);

    warn!("TODO: create file [./{}/schema.yml].", args.collection_name);

    warn!("TODO: create entry for {}.", args.collection_name);

    if !args.no_edit {
        let editor = resolve_editor(&args.editor_opt);
        warn!("TODO: open file [./{}/schema.yml] with {editor}.", args.collection_name);
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







