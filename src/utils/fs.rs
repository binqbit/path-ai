use std::fs;
use serde::{Serialize, Deserialize};



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DirTreeNode {
    dir: String,
    items: Vec<DirTreeNode>,
}


fn dir_tree(path: &str, depth: usize) -> Vec<DirTreeNode> {
    let mut list = Vec::new();
    if depth == 0 || !fs::metadata(path).map(|metadata| metadata.is_dir()).unwrap_or(false) {
        return list;
    }
    if let Ok(files) = fs::read_dir(path) {
        for entry in files.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    if let Some(file_name) = entry.file_name().to_str() {
                        list.push(DirTreeNode {
                            dir: file_name.to_string(),
                            items: dir_tree(&format!("{}/{}", path, file_name), depth - 1),
                        });
                    }
                }
            }
        }
    }
    list
}

pub fn indexed_tree(indexes: Vec<&str>) -> Vec<DirTreeNode> {
    let mut list = Vec::new();
    for index in indexes {
        if index.ends_with('*') {
            let data: Vec<&str> = index.split("::").collect();
            let path = data[0];
            let depth = data[1][..data[1].len() - 1].parse().unwrap_or(0);
            list.push(DirTreeNode {
                dir: path.to_string(),
                items: dir_tree(path, depth),
            });
        } else {
            list.push(DirTreeNode {
                dir: index.to_string(),
                items: Vec::new(),
            });
        }
    }
    list
}