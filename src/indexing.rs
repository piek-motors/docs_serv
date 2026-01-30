use regex::Regex;
use std::{collections::HashMap, error::Error, path::Path, time::Duration};

use crate::{
    api::AppState,
    fs::{FileRef, collect_files_recursive},
};

pub type Index = HashMap<String, FileRef>;

/// Extracts the numeric ID from a filename like "ВЗИС.123.456_somefile.pdf"
/// Returns None if the filename doesn't start with a ВЗИС ID.
pub fn extract_file_id(file_name: &str) -> Option<String> {
    let id_re = Regex::new(r"^ВЗИС\.(\d+\.\d+)").unwrap(); // capture only numbers
    id_re.captures(file_name).map(|caps| caps[1].to_string())
}

pub fn index_documents(path: &Path) -> Result<Index, Box<dyn Error + Send + Sync>> {
    let files = collect_files_recursive(path)?;
    let mut index: Index = HashMap::new();

    for each in files {
        let file_id = extract_file_id(&each.name);
        match file_id {
            Some(id) => {
                index.insert(id, each);
            }
            None => {}
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_file_id() {
        assert_eq!(
            extract_file_id("ВЗИС.123.456_report.pdf"),
            Some("123.456".to_string())
        );
        assert_eq!(
            extract_file_id("ВЗИС.123.456 report.pdf"),
            Some("123.456".to_string())
        );
        assert_eq!(
            extract_file_id("ВЗИС.987.654_some_file.txt"),
            Some("987.654".to_string())
        );
        assert_eq!(
            extract_file_id("ВЗИС.1.2-another_file.doc"),
            Some("1.2".to_string())
        );
        assert_eq!(extract_file_id("report.pdf"), None);
        assert_eq!(extract_file_id("file_ВЗИС.12.34.pdf"), None);
        // Epty string → None
        assert_eq!(extract_file_id(""), None);
    }
}
