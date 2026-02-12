use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use std::path::PathBuf;

pub fn render(
    f: &mut Frame,
    area: Rect,
    themes: &[(String, PathBuf)],
    current_index: usize,
    active_name: Option<&str>,
) {
    let mut spans: Vec<Span> = Vec::new();

    spans.push(Span::styled(" < ", Style::default().fg(Color::DarkGray)));

    for (i, (name, _)) in themes.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled("  \u{2502}  ", Style::default().fg(Color::DarkGray)));
        }

        let is_active = active_name == Some(name.as_str());
        let label = if is_active {
            format!("{}*", name)
        } else {
            name.clone()
        };

        let style = if i == current_index {
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD)
        } else if is_active {
            Style::default().fg(Color::White)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        spans.push(Span::styled(label, style));
    }

    spans.push(Span::styled(" > ", Style::default().fg(Color::DarkGray)));

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray))
        .title(" Themes ");

    let paragraph = Paragraph::new(Line::from(spans)).block(block);
    f.render_widget(paragraph, area);
}
