use crate::config::theme::UserTheme;
use crate::config::types::AnsiColor;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem},
    Frame,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldSelection {
    Enabled = 0,
    StyleMode = 1,
    PlainIcon = 2,
    NerdFontIcon = 3,
    IconColor = 4,
    TextColor = 5,
    BackgroundColor = 6,
    Bold = 7,
}

impl FieldSelection {
    pub fn count() -> usize {
        8
    }

    pub fn from_index(i: usize) -> Self {
        match i {
            0 => Self::Enabled,
            1 => Self::StyleMode,
            2 => Self::PlainIcon,
            3 => Self::NerdFontIcon,
            4 => Self::IconColor,
            5 => Self::TextColor,
            6 => Self::BackgroundColor,
            7 => Self::Bold,
            _ => Self::Enabled,
        }
    }
}

pub struct EditorWidget;

impl EditorWidget {
    pub fn render(
        f: &mut Frame,
        area: Rect,
        theme: &UserTheme,
        selected_component: usize,
        is_focused: bool,
        selected_field: FieldSelection,
    ) {
        let comp = match theme.components.get(selected_component) {
            Some(c) => c,
            None => return,
        };

        let fields: Vec<(&str, String)> = vec![
            ("Enabled", if comp.enabled { "Yes" } else { "No" }.into()),
            ("Style Mode", theme.style.mode.display_name().into()),
            ("Plain Icon", comp.icon.plain.clone()),
            ("Nerd Font Icon", comp.icon.nerd_font.clone()),
            ("Icon Color", format_color(comp.colors.icon.as_ref())),
            ("Text Color", format_color(comp.colors.text.as_ref())),
            ("Bg Color", format_color(comp.colors.background.as_ref())),
            (
                "Bold",
                if comp.styles.text_bold { "Yes" } else { "No" }.into(),
            ),
        ];

        let items: Vec<ListItem> = fields
            .iter()
            .enumerate()
            .map(|(i, (label, value))| {
                let field = FieldSelection::from_index(i);
                let is_selected = field == selected_field && is_focused;

                let style = if is_selected {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Gray)
                };

                let cursor = if is_selected { "> " } else { "  " };

                ListItem::new(Line::from(vec![
                    Span::styled(cursor, style),
                    Span::styled(format!("{}: ", label), style),
                    Span::styled(value.clone(), Style::default().fg(Color::White)),
                ]))
            })
            .collect();

        let border_style = if is_focused {
            Style::default().fg(Color::Blue)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let title = format!(" {} ", comp.id.display_name());
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(border_style)
            .title(title);

        let list = List::new(items).block(block);
        f.render_widget(list, area);
    }
}

fn format_color(color: Option<&AnsiColor>) -> String {
    match color {
        Some(c) => c.to_string(),
        None => "\u{2014}".into(), // —
    }
}
