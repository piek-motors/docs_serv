use regex::Regex;
use std::{collections::HashMap, error::Error, path::Path, time::Duration};

use crate::{
    api::AppState,
    fs::{FileRef, collect_files_recursive},
};

pub type Index = HashMap<String, FileRef>;

pub fn index_documents(path: &Path) -> Result<Index, Box<dyn Error + Send + Sync>> {
    let files = collect_files_recursive(path)?;
    let mut index: Index = HashMap::new();

    let id_re = Regex::new(r"^(ВЗИС\.\d+\.\d+)")?;

    for each in files {
        let name = &each.name;
        if let Some(captures) = id_re.captures(name) {
            let doc_id = captures[1].to_string();
            index.insert(doc_id, each);
        }
    }

    Ok(index)
}

pub fn rebuild_index_task(state: AppState, reindex_interval: Duration) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(reindex_interval);
        interval.tick().await;

        loop {
            interval.tick().await;

            let root_path = state.root_path.clone();
            match index_documents(&root_path) {
                Ok(new_index) => {
                    let mut index_guard = state.index.write().await;
                    *index_guard = new_index;
                    tracing::info!("Index successfully updated");
                }
                Err(e) => {
                    tracing::error!("Failed to rebuild index: {:?}", e);
                }
            }
        }
    });
}
