use std::path::{Path, PathBuf};

use ratatui::text::Line;
use syntect::{highlighting::ThemeSet, parsing::SyntaxSet};

use crate::file_tree::{EntryKind, FileTree};
use crate::markdown::{self, HeadingInfo};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Focus {
    #[default]
    FileTree,
    Toc,
    Content,
}

pub struct AppState {
    pub should_quit: bool,
    pub focus: Focus,

    // File tree
    pub tree: FileTree,
    pub tree_selected: usize,
    pub tree_scroll: usize,

    // TOC
    pub toc_headings: Vec<HeadingInfo>,
    pub toc_selected: usize,
    pub toc_scroll: usize,

    // Content
    pub content_lines: Vec<Line<'static>>,
    pub content_scroll: usize,
    pub current_file: Option<PathBuf>,

    // Layout
    pub sidebar_width: u16,

    // Syntect (loaded once)
    pub syntax_set: SyntaxSet,
    pub theme_set: ThemeSet,

    // Error display
    pub error_message: Option<String>,
}

impl AppState {
    pub fn new(root: &Path) -> Self {
        let tree = FileTree::scan(root);
        Self {
            should_quit: false,
            focus: Focus::FileTree,
            tree,
            tree_selected: 0,
            tree_scroll: 0,
            toc_headings: Vec::new(),
            toc_selected: 0,
            toc_scroll: 0,
            content_lines: Vec::new(),
            content_scroll: 0,
            current_file: None,
            sidebar_width: 30,
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
            error_message: None,
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn toggle_focus(&mut self) {
        self.focus = match self.focus {
            Focus::FileTree => Focus::Toc,
            Focus::Toc => Focus::Content,
            Focus::Content => Focus::FileTree,
        };
    }

    // --- Tree navigation ---

    pub fn tree_up(&mut self) {
        if self.tree_selected > 0 {
            self.tree_selected -= 1;
        }
    }

    pub fn tree_down(&mut self) {
        if !self.tree.entries.is_empty() && self.tree_selected < self.tree.entries.len() - 1 {
            self.tree_selected += 1;
        }
    }

    pub fn tree_enter(&mut self) {
        if self.tree.entries.is_empty() {
            return;
        }
        let entry = &self.tree.entries[self.tree_selected];
        match entry.kind {
            EntryKind::Dir => {
                self.tree.toggle_expand(self.tree_selected);
            }
            EntryKind::Md => {
                let path = entry.path.clone();
                self.open_file(&path);
            }
        }
    }

    pub fn tree_collapse(&mut self) {
        if self.tree.entries.is_empty() {
            return;
        }
        let entry = &self.tree.entries[self.tree_selected];
        if entry.kind == EntryKind::Dir && entry.expanded {
            self.tree.toggle_expand(self.tree_selected);
        }
    }

    // --- TOC navigation ---

    pub fn toc_up(&mut self) {
        if self.toc_selected > 0 {
            self.toc_selected -= 1;
        }
    }

    pub fn toc_down(&mut self) {
        if !self.toc_headings.is_empty() && self.toc_selected < self.toc_headings.len() - 1 {
            self.toc_selected += 1;
        }
    }

    pub fn toc_enter(&mut self) {
        if let Some(heading) = self.toc_headings.get(self.toc_selected) {
            self.content_scroll = heading.line_index;
        }
    }

    // --- Content navigation ---

    pub fn content_up(&mut self) {
        if self.content_scroll > 0 {
            self.content_scroll -= 1;
        }
    }

    pub fn content_down(&mut self) {
        if self.content_scroll < self.content_lines.len().saturating_sub(1) {
            self.content_scroll += 1;
        }
    }

    pub fn content_page_up(&mut self, page_size: usize) {
        self.content_scroll = self.content_scroll.saturating_sub(page_size);
    }

    pub fn content_page_down(&mut self, page_size: usize) {
        let max = self.content_lines.len().saturating_sub(1);
        self.content_scroll = (self.content_scroll + page_size).min(max);
    }

    pub fn content_home(&mut self) {
        self.content_scroll = 0;
    }

    pub fn content_end(&mut self) {
        self.content_scroll = self.content_lines.len().saturating_sub(1);
    }

    // --- File opening ---

    fn open_file(&mut self, path: &Path) {
        self.error_message = None;
        match std::fs::read_to_string(path) {
            Ok(content) => {
                let (lines, headings) =
                    markdown::parse_markdown(&content, &self.syntax_set, &self.theme_set);
                self.content_lines = lines;
                self.toc_headings = headings;
                self.toc_selected = 0;
                self.toc_scroll = 0;
                self.content_scroll = 0;
                self.current_file = Some(path.to_path_buf());
                self.focus = Focus::Content;
            }
            Err(e) => {
                self.error_message = Some(format!("Error reading {}: {}", path.display(), e));
                self.content_lines = vec![Line::raw(format!(
                    "Error reading file: {}",
                    e
                ))];
                self.content_scroll = 0;
                self.current_file = Some(path.to_path_buf());
            }
        }
    }
}
