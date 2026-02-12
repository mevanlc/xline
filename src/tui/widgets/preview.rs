use crate::config::theme::UserTheme;
use crate::config::types::{AnsiColor, ComponentId, StyleMode};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub struct PreviewWidget;

impl PreviewWidget {
    pub fn render(f: &mut Frame, area: Rect, theme: &UserTheme) {
        let spans = build_preview_spans(theme);
        let line = Line::from(spans);
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Preview ");
        let paragraph = Paragraph::new(line).block(block);
        f.render_widget(paragraph, area);
    }
}

fn build_preview_spans(theme: &UserTheme) -> Vec<Span<'static>> {
    let mut spans = Vec::new();
    let sep_glyph = theme.separator_glyph().to_string();
    let sep_color = theme
        .separator()
        .and_then(|s| s.colors.icon.as_ref())
        .cloned();

    let enabled: Vec<_> = theme
        .components
        .iter()
        .filter(|c| c.enabled && c.id != ComponentId::Separator)
        .collect();

    for (i, comp) in enabled.iter().enumerate() {
        if i > 0 && !sep_glyph.is_empty() {
            spans.push(Span::styled(
                sep_glyph.clone(),
                ansi_to_style(sep_color.as_ref(), false),
            ));
        }

        let icon = match theme.style.mode {
            StyleMode::Plain => &comp.icon.plain,
            StyleMode::NerdFont | StyleMode::Powerline => &comp.icon.nerd_font,
        };

        let icon_style = ansi_to_style_with_bg(
            comp.colors.icon.as_ref(),
            comp.colors.background.as_ref(),
            false,
        );
        let text_style = ansi_to_style_with_bg(
            comp.colors.text.as_ref(),
            comp.colors.background.as_ref(),
            comp.styles.text_bold,
        );

        // Demo text for each component
        let demo_text = match comp.id {
            ComponentId::Model => "Sonnet 4.5",
            ComponentId::Directory => "project",
            ComponentId::Git => "main \u{2713}",
            ComponentId::ContextWindow => "12% \u{b7} 24k tokens",
            ComponentId::Usage => "45%",
            ComponentId::Cost => "$1.23",
            ComponentId::Session => "5m",
            ComponentId::OutputStyle => "concise",
            ComponentId::Separator => "",
        };

        if comp.colors.background.is_some() {
            spans.push(Span::styled(format!(" {} ", icon), icon_style));
            spans.push(Span::styled(format!("{} ", demo_text), text_style));
        } else {
            spans.push(Span::styled(format!("{} ", icon), icon_style));
            spans.push(Span::styled(demo_text.to_string(), text_style));
        }
    }

    if spans.is_empty() {
        spans.push(Span::raw("(no enabled components)"));
    }

    spans
}

fn ansi_to_style(color: Option<&AnsiColor>, bold: bool) -> Style {
    ansi_to_style_with_bg(color, None, bold)
}

fn ansi_to_style_with_bg(
    fg: Option<&AnsiColor>,
    bg: Option<&AnsiColor>,
    bold: bool,
) -> Style {
    let mut style = Style::default();
    if let Some(c) = fg {
        style = style.fg(ansi_to_ratatui_color(c));
    }
    if let Some(c) = bg {
        style = style.bg(ansi_to_ratatui_color(c));
    }
    if bold {
        style = style.add_modifier(Modifier::BOLD);
    }
    style
}

fn ansi_to_ratatui_color(color: &AnsiColor) -> Color {
    match color {
        AnsiColor::Color16 { c16 } => match c16 {
            0 => Color::Black,
            1 => Color::Red,
            2 => Color::Green,
            3 => Color::Yellow,
            4 => Color::Blue,
            5 => Color::Magenta,
            6 => Color::Cyan,
            7 => Color::Gray,
            8 => Color::DarkGray,
            9 => Color::LightRed,
            10 => Color::LightGreen,
            11 => Color::LightYellow,
            12 => Color::LightBlue,
            13 => Color::LightMagenta,
            14 => Color::LightCyan,
            15 => Color::White,
            _ => Color::Reset,
        },
        AnsiColor::Color256 { c256 } => Color::Indexed(*c256),
        AnsiColor::Rgb { r, g, b } => Color::Rgb(*r, *g, *b),
    }
}
