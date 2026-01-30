use std::{
    fs, io,
    path::{Path, PathBuf},
};

use regex::Regex;
use serde::Serialize;

use crate::indexing::{extract_full_file_id, extract_numeric_file_id};

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum FsNode {
    File { name: String, id: Option<String> },
    Dir { name: String, children: Vec<FsNode> },
}

pub fn read_dir_recursive(path: &Path) -> io::Result<Vec<FsNode>> {
    let mut nodes = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let name = entry.file_name().to_string_lossy().to_string();
        let entry_path = entry.path();

        if file_type.is_dir() {
            let children = read_dir_recursive(&entry_path)?;
            nodes.push(FsNode::Dir { name, children });
        } else if file_type.is_file() {
            let numeric_id_part = extract_numeric_file_id(&name); // e.g., "123.456"
            let full_id_with_prefix = extract_full_file_id(&name); // e.g., "ВЗИС.123.456"

            let polished_name = match full_id_with_prefix {
                Some(full_id) => {
                    let re = Regex::new(&format!(r"^{}[_\-\s]*", regex::escape(&full_id))).unwrap();
                    re.replace(&name, "").to_string()
                }
                None => name.to_string(),
            };

            nodes.push(FsNode::File {
                name: polished_name,
                id: numeric_id_part,
            });
        }
    }

    Ok(nodes)
}

#[derive(Debug)]
pub struct FileRef {
    pub name: String,
    pub path: PathBuf,
}

pub fn collect_files_recursive(path: &Path) -> io::Result<Vec<FileRef>> {
    let mut nodes = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let name = entry.file_name().to_string_lossy().to_string();
        let entry_path = entry.path();

        if file_type.is_dir() {
            let files = collect_files_recursive(&entry_path)?;
            nodes.extend(files);
        } else if file_type.is_file() {
            nodes.push(FileRef {
                name,
                path: entry_path,
            });
        }
    }

    Ok(nodes)
}
