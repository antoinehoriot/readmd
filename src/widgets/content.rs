use std::path::Path;

use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::style::{focused_border, unfocused_border};

pub fn render_content(
    f: &mut Frame,
    area: Rect,
    lines: &[Line<'static>],
    scroll: usize,
    current_file: Option<&Path>,
    focused: bool,
) {
    let title = current_file
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .unwrap_or("Content");

    let border_style = if focused {
        focused_border()
    } else {
        unfocused_border()
    };

    let block = Block::default()
        .title(title.to_owned())
        .borders(Borders::ALL)
        .border_style(border_style);

    let paragraph = Paragraph::new(lines.to_vec())
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((scroll as u16, 0));

    f.render_widget(paragraph, area);
}
