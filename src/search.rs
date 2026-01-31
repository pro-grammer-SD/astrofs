use rayon::prelude::*;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use ignore::WalkBuilder;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SearchResult {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub relevance: usize,
}

pub struct SearchEngine {
    pub results: Vec<SearchResult>,
    pub is_searching: bool,
}

impl SearchEngine {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
            is_searching: false,
        }
    }

    pub fn search_current_dir(&mut self, dir: &Path, query: &str, max_results: usize) {
        if query.is_empty() {
            self.results.clear();
            return;
        }

        let query_lower = query.to_lowercase();
        self.is_searching = true;

        let mut results: Vec<SearchResult> = WalkDir::new(dir)
            .max_depth(10)
            .into_iter()
            .filter_map(|e| e.ok())
            .par_bridge()
            .filter_map(|entry| {
                let path = entry.path();
                let name = path.file_name()?.to_string_lossy().to_string();
                
                // Fuzzy matching with relevance scoring
                let relevance = Self::calculate_relevance(&name, &query_lower);
                
                if relevance > 0 {
                    Some(SearchResult {
                        path: path.to_path_buf(),
                        name,
                        is_dir: path.is_dir(),
                        relevance,
                    })
                } else {
                    None
                }
            })
            .collect();

        // Sort by relevance (higher first)
        results.sort_by(|a, b| b.relevance.cmp(&a.relevance));
        results.truncate(max_results);

        self.results = results;
        self.is_searching = false;
    }

    #[allow(dead_code)]
    pub fn search_entire_drive(&mut self, root: &Path, query: &str, max_results: usize) {
        if query.is_empty() {
            self.results.clear();
            return;
        }

        let query_lower = query.to_lowercase();
        self.is_searching = true;

        // Use ignore crate for faster traversal (respects .gitignore)
        let mut results: Vec<SearchResult> = WalkBuilder::new(root)
            .max_depth(Some(15))
            .hidden(false)
            .build()
            .par_bridge()
            .filter_map(|e| e.ok())
            .filter_map(|entry| {
                let path = entry.path();
                let name = path.file_name()?.to_string_lossy().to_string();
                
                let relevance = Self::calculate_relevance(&name, &query_lower);
                
                if relevance > 0 {
                    Some(SearchResult {
                        path: path.to_path_buf(),
                        name,
                        is_dir: path.is_dir(),
                        relevance,
                    })
                } else {
                    None
                }
            })
            .collect();

        results.sort_by(|a, b| b.relevance.cmp(&a.relevance));
        results.truncate(max_results);

        self.results = results;
        self.is_searching = false;
    }

    fn calculate_relevance(name: &str, query: &str) -> usize {
        let name_lower = name.to_lowercase();
        
        // Exact match
        if name_lower == query {
            return 1000;
        }
        
        // Starts with query
        if name_lower.starts_with(query) {
            return 500;
        }
        
        // Contains query
        if name_lower.contains(query) {
            return 250;
        }
        
        // Fuzzy match - check if all characters of query appear in order
        let mut query_chars = query.chars();
        let mut current_char = query_chars.next();
        let mut matches = 0;
        
        for c in name_lower.chars() {
            if let Some(qc) = current_char {
                if c == qc {
                    matches += 1;
                    current_char = query_chars.next();
                }
            } else {
                break;
            }
        }
        
        if current_char.is_none() {
            // All characters matched
            100 + matches * 10
        } else {
            0
        }
    }

    #[allow(dead_code)]
    pub fn filter_by_extension(&mut self, extension: &str) {
        let ext_lower = extension.to_lowercase();
        self.results.retain(|r| {
            r.path
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e.to_lowercase() == ext_lower)
                .unwrap_or(false)
        });
    }

    pub fn clear(&mut self) {
        self.results.clear();
    }
}
