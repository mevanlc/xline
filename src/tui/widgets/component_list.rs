use crate::config::theme::UserTheme;
use crate::config::types::ComponentId;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem},
    Frame,
};

pub struct ComponentListWidget;

impl ComponentListWidget {
    pub fn render(
        f: &mut Frame,
        area: Rect,
        theme: &UserTheme,
        selected: usize,
        is_focused: bool,
    ) {
        let mut items: Vec<ListItem> = Vec::new();
        for (i, comp) in theme.components.iter().enumerate() {
            if comp.id == ComponentId::Separator {
                items.push(ListItem::new(Line::from(Span::styled(
                    " \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}",
                    Style::default().fg(Color::DarkGray),
                ))));
            }

            let check = if comp.enabled { "\u{2713}" } else { " " };
            let is_sel = i == selected;
            let cursor = if is_sel { ">" } else { " " };

            let style = if is_sel && is_focused {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else if is_sel {
                Style::default().fg(Color::White)
            } else {
                Style::default().fg(Color::Gray)
            };

            items.push(ListItem::new(Line::from(Span::styled(
                format!("{}[{}] {}", cursor, check, comp.id.display_name()),
                style,
            ))));
        }

        let border_style = if is_focused {
            Style::default().fg(Color::Blue)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(border_style)
            .title(" Components ");

        let list = List::new(items).block(block);
        f.render_widget(list, area);
    }
}
