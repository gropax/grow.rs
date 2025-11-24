use std::path::{PathBuf};
use tracing::{debug, error, info, warn};
use crate::utils;

#[derive(Debug)]
pub struct InitArgs {
    pub collection_name: String,
    pub directory: PathBuf,
    pub editor_opt: Option<String>,
    pub no_edit: bool,
    pub verbose: u8,
}

pub fn handle_init(args: &InitArgs) {
    utils::init_tracing(args.verbose);

    utils::ensure_data_dir();

    let collection_dir = args.directory.join(&args.collection_name);
    debug!("Using collection directory: {}", collection_dir.display());
    utils::ensure_dir(&collection_dir);

    let documents_dir = collection_dir.join("documents");
    debug!("Using documents directory: {}", documents_dir.display());
    utils::ensure_dir(&documents_dir);

    warn!("TODO: create file [./{}/schema.yml].", args.collection_name);

    warn!("TODO: create entry for {}.", args.collection_name);

    if !args.no_edit {
        let editor = utils::resolve_editor(&args.editor_opt);
        warn!("TODO: open file [./{}/schema.yml] with {editor}.", args.collection_name);
    }
}
