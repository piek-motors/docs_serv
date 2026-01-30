use std::{
    fs, io,
    path::{Path, PathBuf},
};

use serde::Serialize;

use crate::indexing::extract_file_id;

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
            let id = extract_file_id(&name);
            nodes.push(FsNode::File { name, id });
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
