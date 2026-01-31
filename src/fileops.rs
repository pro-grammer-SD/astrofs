use anyhow::{anyhow, Result};
use std::fs;
use std::path::{Path, PathBuf};
use fs_extra::dir::CopyOptions;

pub struct FileOperation;

impl FileOperation {
    /// Copy a file or directory to a new location
    pub fn copy(src: &Path, dest: &Path) -> Result<()> {
        if !src.exists() {
            return Err(anyhow!("Source path does not exist: {:?}", src));
        }

        if src.is_dir() {
            fs_extra::dir::copy(src, dest, &CopyOptions::new().copy_inside(true))
                .map_err(|e| anyhow!("Failed to copy directory: {}", e))?;
        } else {
            fs::copy(src, dest).map_err(|e| anyhow!("Failed to copy file: {}", e))?;
        }
        Ok(())
    }

    /// Move (rename or move to different directory) a file or directory
    pub fn move_path(src: &Path, dest: &Path) -> Result<()> {
        if !src.exists() {
            return Err(anyhow!("Source path does not exist: {:?}", src));
        }

        // Ensure parent directory exists
        if let Some(parent) = dest.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| anyhow!("Failed to create destination directory: {}", e))?;
            }
        }

        fs::rename(src, dest).map_err(|e| anyhow!("Failed to move file: {}", e))?;
        Ok(())
    }

    /// Delete a file or directory recursively
    pub fn delete(path: &Path) -> Result<()> {
        if !path.exists() {
            return Err(anyhow!("Path does not exist: {:?}", path));
        }

        if path.is_dir() {
            fs::remove_dir_all(path).map_err(|e| anyhow!("Failed to delete directory: {}", e))?;
        } else {
            fs::remove_file(path).map_err(|e| anyhow!("Failed to delete file: {}", e))?;
        }
        Ok(())
    }

    /// Rename a file or directory
    pub fn rename(src: &Path, new_name: &str) -> Result<PathBuf> {
        if !src.exists() {
            return Err(anyhow!("Source path does not exist: {:?}", src));
        }

        let parent = src.parent().ok_or_else(|| anyhow!("Cannot get parent directory"))?;
        let dest = parent.join(new_name);

        fs::rename(src, &dest).map_err(|e| anyhow!("Failed to rename: {}", e))?;
        Ok(dest)
    }

    /// Create a new file
    pub fn create_file(path: &Path) -> Result<()> {
        if path.exists() {
            return Err(anyhow!("File already exists: {:?}", path));
        }

        fs::File::create(path).map_err(|e| anyhow!("Failed to create file: {}", e))?;
        Ok(())
    }

    /// Create a new directory
    pub fn create_directory(path: &Path) -> Result<()> {
        if path.exists() {
            return Err(anyhow!("Directory already exists: {:?}", path));
        }

        fs::create_dir_all(path).map_err(|e| anyhow!("Failed to create directory: {}", e))?;
        Ok(())
    }

    /// Get the size of a file or directory
    pub fn get_size(path: &Path) -> Result<u64> {
        if path.is_dir() {
            let mut total_size = 0u64;
            for entry in walkdir::WalkDir::new(path)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if entry.path().is_file() {
                    total_size += fs::metadata(entry.path())?.len();
                }
            }
            Ok(total_size)
        } else {
            Ok(fs::metadata(path)?.len())
        }
    }

    /// Check if a path is safe to operate on (not outside allowed areas)
    pub fn is_path_safe(path: &Path) -> bool {
        // Prevent operations on system critical paths
        let path_str = path.to_string_lossy().to_lowercase();
        
        #[cfg(target_os = "windows")]
        {
            let critical = ["windows", "system32", "drivers", "program files\\system"];
            critical.iter().any(|c| path_str.contains(c))
        }
        
        #[cfg(target_os = "linux")]
        {
            let critical = ["/sys", "/proc", "/dev", "/boot"];
            critical.iter().any(|c| path_str.starts_with(c))
        }
        
        #[cfg(target_os = "macos")]
        {
            let critical = ["/system", "/library/system", "/library/caches"];
            critical.iter().any(|c| path_str.contains(c))
        }
        
        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_create_file() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("test.txt");
        
        FileOperation::create_file(&file_path)?;
        assert!(file_path.exists());
        
        Ok(())
    }

    #[test]
    fn test_create_directory() -> Result<()> {
        let dir = tempdir()?;
        let dir_path = dir.path().join("testdir");
        
        FileOperation::create_directory(&dir_path)?;
        assert!(dir_path.exists());
        
        Ok(())
    }
}
