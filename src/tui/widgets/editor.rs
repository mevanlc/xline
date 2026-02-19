use crate::config::theme::UserTheme;
use crate::config::types::AnsiColor;
use crate::core::render;
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

        let fields: Vec<(&str, String, Option<Color>)> = vec![
            ("Enabled", if comp.enabled { "Yes" } else { "No" }.into(), None),
            ("Style Mode", theme.style.mode.display_name().into(), None),
            ("Plain Icon", comp.icon.plain.clone(), None),
            ("Nerd Icon", comp.icon.nerd_font.clone(), None),
            ("Icon Color", format_color(comp.colors.icon.as_ref()), swatch_color(comp.colors.icon.as_ref())),
            ("Text Color", format_color(comp.colors.text.as_ref()), swatch_color(comp.colors.text.as_ref())),
            ("Bg Color", format_color(comp.colors.background.as_ref()), swatch_color(comp.colors.background.as_ref())),
            ("Bold", if comp.styles.text_bold { "Yes" } else { "No" }.into(), None),
        ];

        let label_col_width = fields.iter().map(|(l, _, _)| l.len()).max().unwrap_or(0) + 1;

        let all_items: Vec<ListItem> = fields
            .iter()
            .enumerate()
            .map(|(i, (label, value, swatch))| {
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

                let mut spans = vec![
                    Span::styled(cursor, style),
                    Span::styled(
                        format!("{:<width$} ", format!("{}:", label), width = label_col_width),
                        style,
                    ),
                    Span::styled(value.clone(), Style::default().fg(Color::White)),
                ];

                if let Some(color) = swatch {
                    spans.push(Span::raw(" "));
                    spans.push(Span::styled("\u{2588}\u{2588}", Style::default().fg(*color)));
                }

                ListItem::new(Line::from(spans))
            })
            .collect();

        let total = all_items.len();
        let selected_idx = selected_field as usize;
        let inner_height = area.height.saturating_sub(2) as usize; // borders

        let items = if total <= inner_height {
            all_items
        } else {
            // Both arrows always shown; visible slots = inner_height - 2
            let visible = inner_height.saturating_sub(2);
            let half = visible / 2;
            let raw_offset = selected_idx.saturating_sub(half);
            let max_offset = total.saturating_sub(visible);
            let offset = raw_offset.min(max_offset);

            let has_above = offset > 0;
            let has_below = offset + visible < total;
            let arrow_active = Style::default().fg(Color::DarkGray);
            let arrow_inactive = Style::default().fg(Color::Rgb(40, 40, 40));

            let mut visible_items: Vec<ListItem> = Vec::new();
            visible_items.push(ListItem::new(Line::from(Span::styled(
                " \u{2191}",
                if has_above { arrow_active } else { arrow_inactive },
            ))));
            visible_items.extend(
                all_items.into_iter().skip(offset).take(visible),
            );
            visible_items.push(ListItem::new(Line::from(Span::styled(
                " \u{2193}",
                if has_below { arrow_active } else { arrow_inactive },
            ))));
            visible_items
        };

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

fn swatch_color(color: Option<&AnsiColor>) -> Option<Color> {
    color.map(render::ansi_to_ratatui_color)
}
