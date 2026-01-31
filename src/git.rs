use git2::{Repository, StatusOptions};
use std::path::Path;

pub struct GitInfo {
    pub branch: Option<String>,
    pub is_dirty: bool,
    pub ahead: usize,
    pub behind: usize,
}

impl GitInfo {
    pub fn new() -> Self {
        Self {
            branch: None,
            is_dirty: false,
            ahead: 0,
            behind: 0,
        }
    }

    pub fn from_path(path: &Path) -> Self {
        match Repository::discover(path) {
            Ok(repo) => {
                let mut info = GitInfo::new();

                // Get branch name
                if let Ok(head) = repo.head() {
                    if let Some(name) = head.shorthand() {
                        info.branch = Some(name.to_string());
                    }
                }

                // Check if working directory is dirty
                let mut opts = StatusOptions::new();
                opts.include_untracked(true);
                
                if let Ok(statuses) = repo.statuses(Some(&mut opts)) {
                    info.is_dirty = !statuses.is_empty();
                }

                // Get ahead/behind info
                if let Ok(head) = repo.head() {
                    if let Some(upstream_name) = head.shorthand() {
                        let branch_name = format!("refs/heads/{}", upstream_name);
                        if let Ok(local_branch) = repo.find_reference(&branch_name) {
                            if let Ok(upstream) = repo.branch_upstream_name(local_branch.name().unwrap()) {
                                let upstream_str = upstream.as_str().unwrap();
                                if let Some(local_oid) = local_branch.target() {
                                    if let Ok(upstream_ref) = repo.find_reference(upstream_str) {
                                        if let Some(upstream_oid) = upstream_ref.target() {
                                            if let Ok((ahead, behind)) = repo.graph_ahead_behind(local_oid, upstream_oid) {
                                                info.ahead = ahead;
                                                info.behind = behind;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                info
            }
            Err(_) => GitInfo::new(),
        }
    }

    pub fn status_string(&self) -> String {
        match &self.branch {
            Some(branch) => {
                let mut status = format!(" {}", branch);
                
                if self.is_dirty {
                    status.push_str(" ✗");
                } else {
                    status.push_str(" ✓");
                }
                
                if self.ahead > 0 {
                    status.push_str(&format!(" ↑{}", self.ahead));
                }
                
                if self.behind > 0 {
                    status.push_str(&format!(" ↓{}", self.behind));
                }
                
                status
            }
            None => String::new(),
        }
    }

    pub fn icon(&self) -> &'static str {
        if self.branch.is_some() {
            ""
        } else {
            ""
        }
    }
}
