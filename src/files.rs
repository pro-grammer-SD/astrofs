use humansize::{format_size, BINARY};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct FileEntry {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub size: u64,
    pub is_hidden: bool,
}

impl FileEntry {
    pub fn from_path(path: &Path) -> anyhow::Result<Self> {
        let metadata = fs::metadata(path)?;
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        
        let is_hidden = name.starts_with('.');

        Ok(Self {
            path: path.to_path_buf(),
            name,
            is_dir: metadata.is_dir(),
            size: metadata.len(),
            is_hidden,
        })
    }

    pub fn size_formatted(&self) -> String {
        if self.is_dir {
            String::from("<DIR>")
        } else {
            format_size(self.size, BINARY)
        }
    }
}

pub fn list_directory(path: &Path, show_hidden: bool) -> anyhow::Result<Vec<FileEntry>> {
    let mut entries = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        
        if let Ok(file_entry) = FileEntry::from_path(&path) {
            if !show_hidden && file_entry.is_hidden {
                continue;
            }
            entries.push(file_entry);
        }
    }

    // Sort: directories first, then alphabetically
    entries.sort_by(|a, b| {
        match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        }
    });

    Ok(entries)
}
