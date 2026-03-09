use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};
use syntect::{
    highlighting::{ThemeSet, Style as SynStyle},
    parsing::SyntaxSet,
    easy::HighlightLines,
};

#[derive(Debug, Clone)]
pub struct HeadingInfo {
    pub title: String,
    pub level: u8,
    pub line_index: usize,
}

use crate::style;

pub fn parse_markdown(
    source: &str,
    syntax_set: &SyntaxSet,
    theme_set: &ThemeSet,
) -> (Vec<Line<'static>>, Vec<HeadingInfo>) {
    let options = Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES;
    let parser = Parser::new_ext(source, options);

    let mut lines: Vec<Line<'static>> = Vec::new();
    let mut headings: Vec<HeadingInfo> = Vec::new();
    let mut current_spans: Vec<Span<'static>> = Vec::new();
    // State tracking
    let mut in_heading: Option<u8> = None;
    let mut heading_text = String::new();
    let mut in_bold = false;
    let mut in_italic = false;
    let mut in_code_block = false;
    let mut code_block_lang: Option<String> = None;
    let mut code_block_content = String::new();
    let mut in_blockquote = false;
    let mut list_stack: Vec<Option<u64>> = Vec::new(); // None = unordered, Some(n) = ordered
    let mut in_link = false;
    let mut in_list_item = false;

    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                let lvl = match level {
                    HeadingLevel::H1 => 1,
                    HeadingLevel::H2 => 2,
                    HeadingLevel::H3 => 3,
                    HeadingLevel::H4 => 4,
                    HeadingLevel::H5 => 5,
                    HeadingLevel::H6 => 6,
                };
                in_heading = Some(lvl);
                let heading_style = Style::default()
                    .fg(style::heading_color(lvl))
                    .add_modifier(Modifier::BOLD);

                // Add heading prefix
                let prefix = "#".repeat(lvl as usize) + " ";
                current_spans.push(Span::styled(prefix, heading_style));
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some(lvl) = in_heading {
                    headings.push(HeadingInfo {
                        title: heading_text.clone(),
                        level: lvl,
                        line_index: lines.len(),
                    });
                    heading_text.clear();
                }
                flush_line(&mut lines, &mut current_spans);
                lines.push(Line::default()); // blank line after heading
                in_heading = None;
            }

            Event::Start(Tag::Paragraph) => {}
            Event::End(TagEnd::Paragraph) => {
                flush_line(&mut lines, &mut current_spans);
                if !in_list_item {
                    lines.push(Line::default());
                }
            }

            Event::Start(Tag::Strong) => {
                in_bold = true;
            }
            Event::End(TagEnd::Strong) => {
                in_bold = false;
            }

            Event::Start(Tag::Emphasis) => {
                in_italic = true;
            }
            Event::End(TagEnd::Emphasis) => {
                in_italic = false;
            }

            Event::Start(Tag::Link { dest_url, .. }) => {
                in_link = true;
                let _ = dest_url; // We style the text, not show the URL inline
            }
            Event::End(TagEnd::Link) => {
                in_link = false;
            }

            Event::Start(Tag::BlockQuote(_)) => {
                in_blockquote = true;
            }
            Event::End(TagEnd::BlockQuote(_)) => {
                in_blockquote = false;
                flush_line(&mut lines, &mut current_spans);
                lines.push(Line::default());
            }

            Event::Start(Tag::List(start)) => {
                list_stack.push(start);
            }
            Event::End(TagEnd::List(_)) => {
                list_stack.pop();
                if list_stack.is_empty() {
                    lines.push(Line::default());
                }
            }

            Event::Start(Tag::Item) => {
                in_list_item = true;
                let indent = "  ".repeat(list_stack.len().saturating_sub(1));
                let bullet = if let Some(Some(n)) = list_stack.last() {
                    let s = format!("{indent}{}. ", n);
                    // Increment the counter
                    if let Some(Some(ref mut counter)) = list_stack.last_mut() {
                        *counter += 1;
                    }
                    s
                } else {
                    format!("{indent}  \u{2022} ")
                };
                current_spans.push(Span::raw(bullet));
            }
            Event::End(TagEnd::Item) => {
                flush_line(&mut lines, &mut current_spans);
                in_list_item = false;
            }

            Event::Start(Tag::CodeBlock(kind)) => {
                in_code_block = true;
                code_block_content.clear();
                code_block_lang = match kind {
                    pulldown_cmark::CodeBlockKind::Fenced(lang) => {
                        let l = lang.to_string();
                        if l.is_empty() { None } else { Some(l) }
                    }
                    pulldown_cmark::CodeBlockKind::Indented => None,
                };
            }
            Event::End(TagEnd::CodeBlock) => {
                // Highlight and emit code block
                let highlighted = highlight_code(
                    &code_block_content,
                    code_block_lang.as_deref(),
                    syntax_set,
                    theme_set,
                );
                lines.extend(highlighted);
                lines.push(Line::default());
                in_code_block = false;
                code_block_lang = None;
                code_block_content.clear();
            }

            Event::Code(text) => {
                let s = Style::default().fg(style::INLINE_CODE);
                current_spans.push(Span::styled(
                    format!("`{}`", text),
                    s,
                ));
            }

            Event::Text(text) => {
                if in_heading.is_some() {
                    heading_text.push_str(&text);
                }
                if in_code_block {
                    code_block_content.push_str(&text);
                } else {
                    let s = compute_style(in_heading, in_bold, in_italic, in_link, in_blockquote);

                    if in_blockquote && current_spans.is_empty() {
                        current_spans.push(Span::styled(
                            "\u{2502} ",
                            Style::default().fg(style::BLOCKQUOTE_BAR),
                        ));
                    }

                    // Handle multi-line text
                    let text_str = text.to_string();
                    let mut parts = text_str.split('\n');
                    if let Some(first) = parts.next() {
                        if !first.is_empty() {
                            current_spans.push(Span::styled(first.to_string(), s));
                        }
                    }
                    for part in parts {
                        flush_line(&mut lines, &mut current_spans);
                        if in_blockquote {
                            current_spans.push(Span::styled(
                                "\u{2502} ",
                                Style::default().fg(style::BLOCKQUOTE_BAR),
                            ));
                        }
                        if !part.is_empty() {
                            current_spans.push(Span::styled(part.to_string(), s));
                        }
                    }
                }
            }

            Event::SoftBreak => {
                current_spans.push(Span::raw(" "));
            }

            Event::HardBreak => {
                flush_line(&mut lines, &mut current_spans);
            }

            Event::Rule => {
                flush_line(&mut lines, &mut current_spans);
                lines.push(Line::from(Span::styled(
                    "\u{2500}".repeat(80),
                    Style::default().fg(style::HORIZONTAL_RULE),
                )));
                lines.push(Line::default());
            }

            _ => {}
        }
    }

    // Flush any remaining spans
    if !current_spans.is_empty() {
        flush_line(&mut lines, &mut current_spans);
    }

    (lines, headings)
}

fn flush_line(lines: &mut Vec<Line<'static>>, spans: &mut Vec<Span<'static>>) {
    if spans.is_empty() {
        return;
    }
    lines.push(Line::from(std::mem::take(spans)));
}

fn compute_style(
    in_heading: Option<u8>,
    in_bold: bool,
    in_italic: bool,
    in_link: bool,
    in_blockquote: bool,
) -> Style {
    let mut s = Style::default();

    if let Some(lvl) = in_heading {
        s = s.fg(style::heading_color(lvl)).add_modifier(Modifier::BOLD);
    }
    if in_bold {
        s = s.add_modifier(Modifier::BOLD);
    }
    if in_italic || in_blockquote {
        s = s.add_modifier(Modifier::ITALIC);
    }
    if in_link {
        s = s.fg(style::LINK).add_modifier(Modifier::UNDERLINED);
    }

    s
}

fn highlight_code(
    code: &str,
    lang: Option<&str>,
    syntax_set: &SyntaxSet,
    theme_set: &ThemeSet,
) -> Vec<Line<'static>> {
    let syntax = lang
        .and_then(|l| syntax_set.find_syntax_by_token(l))
        .unwrap_or_else(|| syntax_set.find_syntax_plain_text());

    let theme = &theme_set.themes["base16-ocean.dark"];
    let mut highlighter = HighlightLines::new(syntax, theme);
    let mut result = Vec::new();

    for line in code.lines() {
        let ranges = highlighter
            .highlight_line(line, syntax_set)
            .unwrap_or_default();

        let spans: Vec<Span<'static>> = ranges
            .into_iter()
            .map(|(syn_style, text)| {
                Span::styled(text.to_string(), syntect_to_ratatui(syn_style))
            })
            .collect();

        if spans.is_empty() {
            result.push(Line::raw(String::new()));
        } else {
            result.push(Line::from(spans));
        }
    }

    result
}

fn syntect_to_ratatui(syn_style: SynStyle) -> Style {
    let fg = syn_style.foreground;
    Style::default().fg(Color::Rgb(fg.r, fg.g, fg.b))
}
