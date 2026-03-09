use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryKind {
    Dir,
    Md,
}

#[derive(Debug, Clone)]
pub struct TreeEntry {
    pub name: String,
    pub path: PathBuf,
    pub kind: EntryKind,
    pub depth: usize,
    pub expanded: bool,
}

#[derive(Debug, Default)]
pub struct FileTree {
    pub entries: Vec<TreeEntry>,
}

impl FileTree {
    /// Scan a directory and build a flattened tree of .md files and dirs containing .md files.
    pub fn scan(root: &Path) -> Self {
        let mut entries = Vec::new();
        Self::scan_dir(root, root, 0, &mut entries);
        FileTree { entries }
    }

    fn scan_dir(_root: &Path, dir: &Path, depth: usize, entries: &mut Vec<TreeEntry>) {
        let mut children: Vec<_> = match std::fs::read_dir(dir) {
            Ok(rd) => rd.filter_map(|e| e.ok()).collect(),
            Err(_) => return,
        };
        children.sort_by(|a, b| {
            let a_is_dir = a.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
            let b_is_dir = b.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
            // Dirs first, then alphabetical
            b_is_dir.cmp(&a_is_dir).then_with(|| {
                a.file_name()
                    .to_string_lossy()
                    .to_lowercase()
                    .cmp(&b.file_name().to_string_lossy().to_lowercase())
            })
        });

        for entry in children {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            // Skip hidden files/dirs
            if name.starts_with('.') {
                continue;
            }

            if path.is_dir() {
                if Self::dir_contains_md(&path) {
                    entries.push(TreeEntry {
                        name,
                        path,
                        kind: EntryKind::Dir,
                        depth,
                        expanded: false,
                    });
                    // Children are not added until expanded
                }
            } else if path
                .extension()
                .map(|e| e == "md" || e == "markdown")
                .unwrap_or(false)
            {
                entries.push(TreeEntry {
                    name,
                    path,
                    kind: EntryKind::Md,
                    depth,
                    expanded: false,
                });
            }
        }
    }

    /// Check if a directory (recursively) contains any .md files.
    fn dir_contains_md(dir: &Path) -> bool {
        let entries = match std::fs::read_dir(dir) {
            Ok(rd) => rd,
            Err(_) => return false,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with('.') {
                continue;
            }
            if path.is_dir() {
                if Self::dir_contains_md(&path) {
                    return true;
                }
            } else if path
                .extension()
                .map(|e| e == "md" || e == "markdown")
                .unwrap_or(false)
            {
                return true;
            }
        }
        false
    }

    /// Toggle expand/collapse for a directory entry at the given index.
    pub fn toggle_expand(&mut self, index: usize) {
        if index >= self.entries.len() {
            return;
        }
        if self.entries[index].kind != EntryKind::Dir {
            return;
        }

        if self.entries[index].expanded {
            self.collapse(index);
        } else {
            self.expand(index);
        }
    }

    fn expand(&mut self, index: usize) {
        self.entries[index].expanded = true;
        let parent_path = self.entries[index].path.clone();
        let parent_depth = self.entries[index].depth;

        let mut children = Vec::new();
        Self::scan_dir(&parent_path, &parent_path, parent_depth + 1, &mut children);

        // Insert children right after the parent
        let insert_pos = index + 1;
        for (i, child) in children.into_iter().enumerate() {
            self.entries.insert(insert_pos + i, child);
        }
    }

    fn collapse(&mut self, index: usize) {
        self.entries[index].expanded = false;
        let parent_depth = self.entries[index].depth;

        // Remove all entries after index that have depth > parent_depth
        let mut remove_count = 0;
        for entry in &self.entries[index + 1..] {
            if entry.depth > parent_depth {
                remove_count += 1;
            } else {
                break;
            }
        }

        self.entries.drain(index + 1..index + 1 + remove_count);
    }
}
