use regex::Regex;
use std::{collections::HashMap, error::Error, path::Path, time::Duration};

use crate::{
    api::AppState,
    fs::{FileRef, collect_files_recursive},
};

pub type Index = HashMap<String, FileRef>;

/// Extracts only the numeric part of a ВЗИС ID from a filename.
/// Matches either "123" or "123.456" from "ВЗИС.123_report.pdf" or "ВЗИС.123.456_file.pdf".
pub fn extract_numeric_file_id(file_name: &str) -> Option<String> {
    let numeric_re = Regex::new(r"^ВЗИС\.(\d+(?:\.\d+)?)").unwrap();
    numeric_re
        .captures(file_name)
        .map(|caps| caps[1].to_string())
}

/// Extracts the full ВЗИС ID from a filename, including the prefix, e.g. "ВЗИС.123.456" or "ВЗИС.123".
/// Returns None if the filename doesn't start with a ВЗИС ID.
pub fn extract_full_file_id(file_name: &str) -> Option<String> {
    let full_re = Regex::new(r"^(ВЗИС\.\d+(?:\.\d+)?)").unwrap();
    full_re.captures(file_name).map(|caps| caps[1].to_string())
}

pub fn index_documents(path: &Path) -> Result<Index, Box<dyn Error + Send + Sync>> {
    let files = collect_files_recursive(path)?;
    let mut index: Index = HashMap::new();

    for each in files {
        let file_id = extract_numeric_file_id(&each.name);
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
            extract_numeric_file_id("ВЗИС.123.456_report.pdf"),
            Some("123.456".to_string())
        );
        assert_eq!(
            extract_numeric_file_id("ВЗИС.123_report.pdf"),
            Some("123".to_string())
        );
        assert_eq!(
            extract_numeric_file_id("ВЗИС.123.456 report.pdf"),
            Some("123.456".to_string())
        );
        assert_eq!(
            extract_numeric_file_id("ВЗИС.987.654_some_file.txt"),
            Some("987.654".to_string())
        );
        assert_eq!(
            extract_numeric_file_id("ВЗИС.1.2-another_file.doc"),
            Some("1.2".to_string())
        );
        assert_eq!(extract_numeric_file_id("report.pdf"), None);
        assert_eq!(extract_numeric_file_id("file_ВЗИС.12.34.pdf"), None);
        assert_eq!(extract_numeric_file_id(""), None);
    }
}
