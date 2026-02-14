use crate::config::theme::UserTheme;
use crate::core::render;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

const MASCOT_COLOR: Color = Color::Rgb(204, 120, 50);

/// Render the Claude Code banner with mascot, then the statusline preview
/// immediately below (no gap), mimicking the real Claude Code startup screen.
pub fn render(f: &mut Frame, area: Rect, theme: &UserTheme) {
    // Banner box: 10 lines (border + 8 content lines)
    // Statusline: 1 line (no border, just indented)
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10), // Banner box
            Constraint::Length(1),  // Statusline (immediately below)
        ])
        .split(area);

    // --- Banner box ---
    let header_content = Text::from(vec![
        Line::from(""),
        Line::from(Span::styled("\u{2590}\u{259b}\u{2588}\u{2588}\u{2588}\u{259c}\u{258c}", Style::default().fg(MASCOT_COLOR))),
        Line::from(Span::styled("\u{259d}\u{259c}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{259b}\u{2598}", Style::default().fg(MASCOT_COLOR))),
        Line::from(Span::styled(" \u{2598}\u{2598} \u{259d}\u{259d}", Style::default().fg(MASCOT_COLOR))),
        Line::from(Span::styled("Opus \u{00b7} Sonnet \u{00b7} Haiku", Style::default().fg(Color::White))),
        Line::from(Span::styled("you@example.com", Style::default().fg(Color::DarkGray))),
        Line::from(Span::styled(
            format!("~/ccxline-v{}", env!("CARGO_PKG_VERSION")),
            Style::default().fg(Color::Cyan),
        )),
    ]);

    let header_box = Paragraph::new(header_content)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_set(border::ROUNDED)
                .border_style(Style::default().fg(Color::DarkGray))
                .title(format!(" Claude Code v2.1.39 "))
                .title_style(Style::default().fg(Color::DarkGray)),
        )
        .alignment(Alignment::Center);
    f.render_widget(header_box, layout[0]);

    // --- Statusline preview (immediately below, no border) ---
    let texts = render::demo_texts_full();
    let line = render::build_render_line(&theme.components, theme.style.mode, &texts);
    let mut spans = vec![Span::raw("  ")]; // indent to match real CC
    spans.extend(render::render_spans(&line));

    let statusline = Paragraph::new(Line::from(spans));
    f.render_widget(statusline, layout[1]);
}

/// Height needed for the banner + statusline.
pub const HEIGHT: u16 = 11;
