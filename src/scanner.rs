use crate::file::FileInfo;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn scan_directory(path: &Path) -> HashMap<u64, Vec<FileInfo>> {
    let entries = fs::read_dir(path).unwrap();

    let mut files: HashMap<u64, Vec<FileInfo>> = HashMap::new();

    for entry in entries {
        let entry = entry.unwrap();

        if !entry.file_type().unwrap().is_file() {
            continue;
        }

        let file_info = FileInfo::from_path(entry.path());

        files
            .entry(file_info.size)
            .or_insert_with(Vec::new)
            .push(file_info);
    }

    files
}
