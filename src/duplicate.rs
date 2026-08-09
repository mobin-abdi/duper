use crate::file::FileInfo;
use std::collections::HashMap;

pub fn find_candidates(files: HashMap<u64, Vec<FileInfo>>) -> Vec<Vec<FileInfo>> {
    let mut candidates = Vec::new();

    for (_, group) in files {
        if group.len() > 1 {
            candidates.push(group);
        }
    }

    candidates
}

pub fn find_duplicates(candidates: Vec<Vec<FileInfo>>) -> Vec<Vec<FileInfo>> {
    let mut duplicates = Vec::new();

    for group in candidates {
        let mut hashes: HashMap<String, Vec<FileInfo>> = HashMap::new();

        for mut file in group {
            let hash = FileInfo::calculate_hash(&file.path).unwrap();

            file.hash = hash.clone();

            hashes.entry(hash).or_insert_with(Vec::new).push(file);
        }

        for (_, group) in hashes {
            if group.len() > 1 {
                duplicates.push(group);
            }
        }
    }

    duplicates
}
