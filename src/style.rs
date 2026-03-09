use ratatui::style::{Color, Modifier, Style};

// Color palette
pub const HEADING1: Color = Color::Cyan;
pub const HEADING2: Color = Color::Green;
pub const HEADING3: Color = Color::Yellow;
pub const HEADING4: Color = Color::Magenta;
pub const HEADING5: Color = Color::Blue;
pub const HEADING6: Color = Color::Red;

pub const INLINE_CODE: Color = Color::LightYellow;
pub const LINK: Color = Color::Blue;
pub const BLOCKQUOTE_BAR: Color = Color::DarkGray;
pub const HORIZONTAL_RULE: Color = Color::DarkGray;
pub const STATUS_BG: Color = Color::DarkGray;
pub const SELECTED_BG: Color = Color::DarkGray;
pub const DIR_COLOR: Color = Color::Blue;
pub const FILE_COLOR: Color = Color::White;

pub fn heading_color(level: u8) -> Color {
    match level {
        1 => HEADING1,
        2 => HEADING2,
        3 => HEADING3,
        4 => HEADING4,
        5 => HEADING5,
        _ => HEADING6,
    }
}

pub fn focused_border() -> Style {
    Style::default().fg(Color::Cyan)
}

pub fn unfocused_border() -> Style {
    Style::default().fg(Color::DarkGray)
}

pub fn status_bar() -> Style {
    Style::default().bg(STATUS_BG).fg(Color::White)
}

pub fn selected() -> Style {
    Style::default().bg(SELECTED_BG).add_modifier(Modifier::BOLD)
}
