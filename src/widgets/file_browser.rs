use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::file_tree::{EntryKind, FileTree};
use crate::style;

pub fn render_file_browser(
    f: &mut Frame,
    area: Rect,
    tree: &FileTree,
    selected: usize,
    scroll: &mut usize,
    focused: bool,
) {
    let border_style = if focused {
        style::focused_border()
    } else {
        style::unfocused_border()
    };

    let block = Block::default()
        .title("Files")
        .borders(Borders::ALL)
        .border_style(border_style);

    let inner = block.inner(area);
    let visible_height = inner.height as usize;

    // Adjust scroll offset so the selected entry stays visible.
    if selected < *scroll {
        *scroll = selected;
    } else if selected >= *scroll + visible_height {
        *scroll = selected.saturating_sub(visible_height.saturating_sub(1));
    }

    let lines: Vec<Line> = tree
        .entries
        .iter()
        .enumerate()
        .skip(*scroll)
        .take(visible_height)
        .map(|(i, entry)| {
            let indent = "  ".repeat(entry.depth);

            let (icon, color) = match entry.kind {
                EntryKind::Dir => {
                    let arrow = if entry.expanded { "\u{25be} " } else { "\u{25b8} " };
                    (arrow, style::DIR_COLOR)
                }
                EntryKind::Md => ("\u{1f4c4} ", style::FILE_COLOR),
            };

            let line_style = if i == selected {
                style::selected()
            } else {
                ratatui::style::Style::default()
            };

            Line::from(vec![
                Span::styled(indent, line_style),
                Span::styled(icon, line_style.fg(color)),
                Span::styled(&entry.name, line_style.fg(color)),
            ])
        })
        .collect();

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
}
