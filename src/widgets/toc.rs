use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::markdown::HeadingInfo;
use crate::style;

pub fn render_toc(
    f: &mut Frame,
    area: Rect,
    headings: &[HeadingInfo],
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
        .title("Contents")
        .borders(Borders::ALL)
        .border_style(border_style);

    let inner = block.inner(area);
    let visible_height = inner.height as usize;

    // Adjust scroll to keep selection visible.
    if selected < *scroll {
        *scroll = selected;
    } else if selected >= *scroll + visible_height {
        *scroll = selected.saturating_sub(visible_height.saturating_sub(1));
    }

    let min_level = headings.iter().map(|h| h.level).min().unwrap_or(1);

    let lines: Vec<Line> = headings
        .iter()
        .enumerate()
        .skip(*scroll)
        .take(visible_height)
        .map(|(i, heading)| {
            let indent = "  ".repeat((heading.level - min_level) as usize);
            let line_style = if i == selected {
                style::selected()
            } else {
                ratatui::style::Style::default()
            };

            let color = style::heading_color(heading.level);
            Line::from(vec![
                Span::styled(indent, line_style),
                Span::styled(heading.title.clone(), line_style.fg(color)),
            ])
        })
        .collect();

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
}
